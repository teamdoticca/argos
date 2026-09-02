use crate::model::{NodeKind, WorkspaceNode};
use crate::path_identity::PathIdentity;
use crate::planner::WatchScope;
use crate::snapshot::{SnapshotState, WorkspaceSnapshot};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplainPathResult {
    pub exists: bool,
    pub node: Option<String>,
    pub watched: bool,
    pub ignored: bool,
    pub scope: Option<String>,
    pub ignore_rule: Option<String>,
    pub ignore_source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplainScopeResult {
    pub included: Vec<String>,
    pub excluded: Vec<String>,
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerHit {
    pub node_id: String,
    pub kind: NodeKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub nodes: usize,
    pub scopes: usize,
    pub ignored_paths: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformInfo {
    pub os: String,
    pub backend: Option<String>,
    pub case_sensitive: bool,
    pub symlink_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthReport {
    pub status: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationIssue {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotValidation {
    pub valid: bool,
    pub issues: Vec<ValidationIssue>,
}

pub fn list_nodes(snapshot: &WorkspaceSnapshot) -> Vec<&WorkspaceNode> {
    snapshot.model.nodes.iter().collect()
}

pub fn list_scopes(snapshot: &WorkspaceSnapshot) -> &[WatchScope] {
    &snapshot.planning.scopes
}

pub fn find_owner(snapshot: &WorkspaceSnapshot, path: &Path) -> Option<OwnerHit> {
    let case_sensitive = snapshot.filesystem.case_sensitive;
    let target = PathIdentity::from_path(path, case_sensitive);
    let mut best: Option<&WorkspaceNode> = None;
    let mut best_len = 0usize;
    for node in &snapshot.model.nodes {
        let root_id = PathIdentity::from_path(&node.root, case_sensitive);
        if target.key.starts_with(&root_id.key) && root_id.key.len() >= best_len {
            best_len = root_id.key.len();
            best = Some(node);
        }
    }
    best.map(|n| OwnerHit {
        node_id: n.id.clone(),
        kind: n.kind,
    })
}

pub fn explain_path(
    snapshot: &WorkspaceSnapshot,
    path: &Path,
    ignored: bool,
    ignore_rule: Option<(String, String)>,
) -> ExplainPathResult {
    let exists = path.exists();
    let owner = find_owner(snapshot, path);
    let scope = owner.as_ref().and_then(|o| {
        snapshot
            .planning
            .scopes
            .iter()
            .find(|s| s.package == o.node_id)
            .map(|s| s.root.clone())
    });
    let watched = owner
        .as_ref()
        .map(|o| {
            snapshot
                .planning
                .scopes
                .iter()
                .any(|s| s.package == o.node_id && path_in_scope(path, s))
        })
        .unwrap_or(false)
        && !ignored;

    ExplainPathResult {
        exists,
        node: owner.map(|o| o.node_id),
        watched,
        ignored,
        scope,
        ignore_rule: ignore_rule.as_ref().map(|(r, _)| r.clone()),
        ignore_source: ignore_rule.map(|(_, s)| s),
    }
}

fn path_in_scope(path: &Path, scope: &WatchScope) -> bool {
    let root = Path::new(&scope.root);
    let Ok(rel) = path.strip_prefix(root) else {
        return false;
    };
    let rel = rel.to_string_lossy().replace('\\', "/");
    scope.watch.iter().any(|w| {
        // "." means the whole package root is in the watch plan.
        if w == "." {
            return true;
        }
        rel == *w || rel.starts_with(&format!("{w}/"))
    })
}

pub fn explain_scope(snapshot: &WorkspaceSnapshot, scope_root: &str) -> ExplainScopeResult {
    let scope = snapshot
        .planning
        .scopes
        .iter()
        .find(|s| s.root == scope_root || s.package == scope_root);
    match scope {
        Some(s) => ExplainScopeResult {
            included: s.watch.clone(),
            excluded: snapshot.planning.ignores.clone(),
            rules: snapshot.planning.ignores.clone(),
        },
        None => ExplainScopeResult {
            included: vec![],
            excluded: vec![],
            rules: vec![],
        },
    }
}

pub fn workspace_health(snapshot: &WorkspaceSnapshot) -> HealthReport {
    let mut warnings = Vec::new();
    for node in &snapshot.model.nodes {
        if !node.root.exists() {
            warnings.push(format!("missing node root: {}", node.id));
        }
    }
    let status = match snapshot.identity.state {
        SnapshotState::Ready if warnings.is_empty() => "healthy",
        SnapshotState::Ready => "degraded",
        SnapshotState::Degraded => "degraded",
        SnapshotState::Recovering => "recovering",
    };
    HealthReport {
        status: status.into(),
        warnings,
    }
}

pub fn snapshot_validate(snapshot: &WorkspaceSnapshot) -> SnapshotValidation {
    let mut issues = Vec::new();
    if snapshot.identity.schema_version == 0 {
        issues.push(ValidationIssue {
            message: "invalid schemaVersion".into(),
        });
    }
    let mut seen = std::collections::HashSet::new();
    for node in &snapshot.model.nodes {
        if !node.root.as_os_str().is_empty() {
            let key = node.root.to_string_lossy().to_lowercase();
            if !seen.insert(key) {
                issues.push(ValidationIssue {
                    message: format!("duplicate path identity: {}", node.id),
                });
            }
        } else {
            issues.push(ValidationIssue {
                message: format!("missing node root: {}", node.id),
            });
        }
    }
    SnapshotValidation {
        valid: issues.is_empty(),
        issues,
    }
}

pub fn platform_info(case_sensitive: bool, backend: Option<String>) -> PlatformInfo {
    let os = if cfg!(windows) {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    };
    PlatformInfo {
        os: os.into(),
        backend,
        case_sensitive,
        symlink_mode: "follow".into(),
    }
}
