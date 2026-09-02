use crate::diagnostics::{platform_info, PlatformInfo};
use crate::discovery::discover_topology;
use crate::ignore_engine::IgnoreEngine;
use crate::path_identity::{detect_case_sensitivity, PathSemantics};
use crate::planner::plan_scopes;
use crate::snapshot::{
    FilesystemSection, IdentitySection, ModelSection, PlanningSection, SnapshotState,
    WorkspaceSnapshot, SCHEMA_VERSION,
};
use crate::{ArgosError, Result};
use parking_lot::RwLock;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct OpenOptions {
    pub symlink_mode: Option<String>,
}

/// Consumer-owned workspace handle. `current_snapshot` is a pointer to an immutable snapshot.
pub struct Workspace {
    root: PathBuf,
    workspace_id: Uuid,
    case_sensitive: bool,
    symlink_mode: String,
    ignore: IgnoreEngine,
    snapshot: RwLock<Arc<WorkspaceSnapshot>>,
    snapshot_version: RwLock<u64>,
    content_version: RwLock<u64>,
    backend_name: RwLock<Option<String>>,
}

impl Workspace {
    pub fn open(root: impl AsRef<Path>, options: OpenOptions) -> Result<Self> {
        let root = root.as_ref();
        if !root.exists() {
            return Err(ArgosError::RootMissing(root.display().to_string()));
        }
        if !root.is_dir() {
            return Err(ArgosError::RootNotDirectory(root.display().to_string()));
        }
        let root = crate::path_identity::canonicalize_lossy(root);
        let case_sensitive = detect_case_sensitivity(&root);
        let symlink_mode = options
            .symlink_mode
            .unwrap_or_else(|| "follow".to_string());
        let workspace_id = Uuid::new_v4();
        let ignore = IgnoreEngine::build(&root, case_sensitive)?;
        let (snapshot, snap_v, content_v) =
            build_snapshot(&root, workspace_id, case_sensitive, &symlink_mode, &ignore, 1, 1, None, None, SnapshotState::Ready)?;

        Ok(Self {
            root,
            workspace_id,
            case_sensitive,
            symlink_mode,
            ignore,
            snapshot: RwLock::new(Arc::new(snapshot)),
            snapshot_version: RwLock::new(snap_v),
            content_version: RwLock::new(content_v),
            backend_name: RwLock::new(None),
        })
    }

    pub fn workspace_id(&self) -> Uuid {
        self.workspace_id
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Pointer to the latest immutable snapshot.
    pub fn current_snapshot(&self) -> Arc<WorkspaceSnapshot> {
        self.snapshot.read().clone()
    }

    pub fn rebuild_snapshot(&self) -> Result<Arc<WorkspaceSnapshot>> {
        let prev_snap = *self.snapshot_version.read();
        let prev_content = *self.content_version.read();
        let old = self.current_snapshot();
        let (mut new_snap, _, _) = build_snapshot(
            &self.root,
            self.workspace_id,
            self.case_sensitive,
            &self.symlink_mode,
            &self.ignore,
            prev_snap + 1,
            prev_content,
            Some(prev_snap),
            Some(prev_content),
            SnapshotState::Ready,
        )?;

        // Bump content version only when business content changes.
        let content_changed = content_fingerprint(&old) != content_fingerprint(&new_snap);
        if content_changed {
            new_snap.identity.content_version = prev_content + 1;
            new_snap.identity.previous_content_version = Some(prev_content);
            *self.content_version.write() = prev_content + 1;
        } else {
            new_snap.identity.content_version = prev_content;
            new_snap.identity.previous_content_version = old.identity.previous_content_version;
        }
        *self.snapshot_version.write() = prev_snap + 1;
        let arc = Arc::new(new_snap);
        *self.snapshot.write() = arc.clone();
        Ok(arc)
    }

    pub fn ignore(&self) -> &IgnoreEngine {
        &self.ignore
    }

    pub fn set_backend_name(&self, name: Option<String>) {
        *self.backend_name.write() = name;
    }

    pub fn platform_info(&self) -> PlatformInfo {
        platform_info(self.case_sensitive, self.backend_name.read().clone())
    }

    pub fn mark_recovering(&self) -> Result<Arc<WorkspaceSnapshot>> {
        let mut snap = (*self.current_snapshot()).clone();
        let prev = snap.identity.snapshot_version;
        snap.identity.previous_snapshot_version = Some(prev);
        snap.identity.snapshot_version = prev + 1;
        snap.identity.state = SnapshotState::Recovering;
        *self.snapshot_version.write() = snap.identity.snapshot_version;
        let arc = Arc::new(snap);
        *self.snapshot.write() = arc.clone();
        Ok(arc)
    }

    pub fn mark_ready_from_rebuild(&self) -> Result<Arc<WorkspaceSnapshot>> {
        let snap = self.rebuild_snapshot()?;
        // rebuild already sets Ready
        Ok(snap)
    }
}

fn build_snapshot(
    root: &Path,
    workspace_id: Uuid,
    case_sensitive: bool,
    symlink_mode: &str,
    ignore: &IgnoreEngine,
    snapshot_version: u64,
    content_version: u64,
    previous_snapshot_version: Option<u64>,
    previous_content_version: Option<u64>,
    state: SnapshotState,
) -> Result<(WorkspaceSnapshot, u64, u64)> {
    let topology = discover_topology(root)?;
    let mut warnings_degraded = false;
    for n in &topology.nodes {
        if !n.root.exists() {
            warnings_degraded = true;
            break;
        }
    }
    let state = if warnings_degraded && matches!(state, SnapshotState::Ready) {
        SnapshotState::Degraded
    } else {
        state
    };

    let scopes = plan_scopes(&topology.nodes, ignore);
    let artifacts: Vec<String> = topology
        .nodes
        .iter()
        .filter(|n| matches!(n.kind, crate::model::NodeKind::OsArtifact))
        .map(|n| format!("{}:{}", n.source.manifest, n.path_identity.display))
        .collect();
    let snap = WorkspaceSnapshot {
        identity: IdentitySection {
            workspace_id,
            schema_version: SCHEMA_VERSION,
            snapshot_version,
            previous_snapshot_version,
            content_version,
            previous_content_version,
            state,
            root: root.to_string_lossy().replace('\\', "/"),
        },
        model: ModelSection {
            nodes: topology.nodes,
        },
        filesystem: FilesystemSection {
            case_sensitive,
            symlink_mode: symlink_mode.to_string(),
            semantics: PathSemantics {
                case_sensitive,
                follows_symlinks: symlink_mode == "follow",
            },
            artifacts,
        },
        planning: PlanningSection {
            ignores: ignore.rule_patterns(),
            scopes,
        },
    };
    Ok((snap, snapshot_version, content_version))
}

fn content_fingerprint(snapshot: &WorkspaceSnapshot) -> String {
    let mut nodes: Vec<_> = snapshot
        .model
        .nodes
        .iter()
        .map(|n| {
            format!(
                "{}:{:?}:{}",
                n.id,
                n.kind,
                n.root.to_string_lossy().replace('\\', "/")
            )
        })
        .collect();
    nodes.sort();
    let mut scopes: Vec<_> = snapshot
        .planning
        .scopes
        .iter()
        .map(|s| format!("{}:{}:{:?}", s.package, s.root, s.watch))
        .collect();
    scopes.sort();
    format!("{nodes:?}|{scopes:?}")
}
