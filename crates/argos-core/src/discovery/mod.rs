mod cargo_ws;
mod git_submodules;
mod npm;
mod os_artifacts;
mod pnpm;
mod workspace_file;

use crate::model::{TopologyResult, WorkspaceNode};
use crate::path_identity::{detect_case_sensitivity, PathIdentity};
use crate::Result;
use std::collections::HashSet;
use std::path::Path;

pub(crate) fn blank_identity() -> PathIdentity {
    PathIdentity {
        key: String::new(),
        display: String::new(),
    }
}

pub fn discover_topology(root: &Path) -> Result<TopologyResult> {
    let mut nodes: Vec<WorkspaceNode> = Vec::new();
    let mut seen = HashSet::new();
    let case_sensitive = detect_case_sensitivity(root);

    let mut push = |mut node: WorkspaceNode| {
        if node.path_identity.key.is_empty() {
            node.path_identity = PathIdentity::from_path(&node.root, case_sensitive);
        }
        let key = node.path_identity.key.clone();
        if seen.insert(key) {
            nodes.push(node);
        }
    };

    for n in pnpm::discover(root)? {
        push(n);
    }
    for n in npm::discover(root)? {
        push(n);
    }
    for n in cargo_ws::discover(root)? {
        push(n);
    }
    for n in git_submodules::discover(root)? {
        push(n);
    }
    for n in workspace_file::discover(root)? {
        push(n);
    }
    for n in os_artifacts::discover(root)? {
        push(n);
    }
    drop(push);

    let has_topology = nodes
        .iter()
        .any(|n| !matches!(n.kind, crate::model::NodeKind::OsArtifact));
    if !has_topology {
        if root.join("package.json").is_file() || root.join("Cargo.toml").is_file() {
            let mut node = WorkspaceNode {
                id: root
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "root".into()),
                kind: crate::model::NodeKind::Package,
                root: root.to_path_buf(),
                path_identity: blank_identity(),
                source: crate::model::DiscoverySource {
                    provider: "heuristic".into(),
                    manifest: "root".into(),
                },
                confidence: crate::model::Confidence::Medium,
            };
            if node.path_identity.key.is_empty() {
                node.path_identity = PathIdentity::from_path(&node.root, case_sensitive);
            }
            let key = node.path_identity.key.clone();
            if seen.insert(key) {
                nodes.push(node);
            }
        }
    }

    Ok(TopologyResult { nodes })
}
