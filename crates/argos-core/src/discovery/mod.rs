mod cargo_ws;
mod document;
mod dotnet;
mod git_submodules;
mod golang;
mod gradle;
mod maven;
mod nested_npm;
mod npm;
mod ops;
mod os_artifacts;
mod php;
mod pnpm;
mod python;
mod walk;
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

pub(crate) fn ops_compose_build_contexts(compose_file: &Path) -> Vec<String> {
    ops::compose_build_contexts(compose_file)
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
    for n in nested_npm::discover(root)? {
        push(n);
    }
    for n in cargo_ws::discover(root)? {
        push(n);
    }
    for n in dotnet::discover(root)? {
        push(n);
    }
    for n in php::discover(root)? {
        push(n);
    }
    for n in python::discover(root)? {
        push(n);
    }
    for n in golang::discover(root)? {
        push(n);
    }
    for n in gradle::discover(root)? {
        push(n);
    }
    for n in maven::discover(root)? {
        push(n);
    }
    for n in git_submodules::discover(root)? {
        push(n);
    }
    for n in workspace_file::discover(root)? {
        push(n);
    }
    for n in ops::discover(root)? {
        push(n);
    }
    // document-root / guidance-path need the package topology for dedupe.
    for mut node in document::discover(root, &nodes)? {
        if node.path_identity.key.is_empty() {
            node.path_identity = PathIdentity::from_path(&node.root, case_sensitive);
        }
        let key = node.path_identity.key.clone();
        if seen.insert(key) {
            nodes.push(node);
        }
    }
    let mut push = |mut node: WorkspaceNode| {
        if node.path_identity.key.is_empty() {
            node.path_identity = PathIdentity::from_path(&node.root, case_sensitive);
        }
        let key = node.path_identity.key.clone();
        if seen.insert(key) {
            nodes.push(node);
        }
    };
    for n in os_artifacts::discover(root)? {
        push(n);
    }

    let has_topology = nodes
        .iter()
        .any(|n| !matches!(n.kind, crate::model::NodeKind::OsArtifact));
    if !has_topology {
        // Last-resort heuristic: root package/cargo/.csproj/.sln markers only.
        let root_csproj = std::fs::read_dir(root)
            .ok()
            .into_iter()
            .flatten()
            .flatten()
            .any(|e| {
                e.path()
                    .extension()
                    .and_then(|x| x.to_str())
                    .is_some_and(|x| {
                        x.eq_ignore_ascii_case("csproj") || x.eq_ignore_ascii_case("sln")
                    })
            });
        if root.join("package.json").is_file() || root.join("Cargo.toml").is_file() || root_csproj {
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

    let has_topology = nodes
        .iter()
        .any(|n| !matches!(n.kind, crate::model::NodeKind::OsArtifact));
    if !has_topology {
        // Explicit documented fallback — never leave Open with silent empty topology.
        // Planner may emit package-root "." watch; ignore engine still applies.
        let mut node = WorkspaceNode {
            id: "workspace-root".into(),
            kind: crate::model::NodeKind::Package,
            root: root.to_path_buf(),
            path_identity: blank_identity(),
            source: crate::model::DiscoverySource {
                provider: "fallback-root".into(),
                manifest: "none".into(),
            },
            confidence: crate::model::Confidence::Low,
        };
        node.path_identity = PathIdentity::from_path(&node.root, case_sensitive);
        let key = node.path_identity.key.clone();
        if seen.insert(key) {
            nodes.push(node);
        }
    }

    Ok(TopologyResult { nodes })
}
