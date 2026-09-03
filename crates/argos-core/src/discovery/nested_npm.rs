use crate::model::{Confidence, DiscoverySource, NodeKind, WorkspaceNode};
use crate::Result;
use std::path::Path;

/// Discover nested npm/yarn/pnpm packages via `package.json` anywhere under the root
/// (not only root workspace manifests).
pub fn discover(root: &Path) -> Result<Vec<WorkspaceNode>> {
    let mut out = Vec::new();
    for path in super::walk::walk_entries(root) {
        if path.file_name().and_then(|s| s.to_str()) != Some("package.json") {
            continue;
        }
        let Some(dir) = path.parent() else {
            continue;
        };
        // Root package.json is handled by npm/pnpm workspace providers + heuristic.
        if dir == root {
            continue;
        }
        // Skip obvious test asset stubs if desired — still discoverable; leave included
        // for parity (ignore engine / planner may no-op empty watches).
        out.push(package_node(dir, &path));
    }
    Ok(out)
}

fn package_node(dir: &Path, manifest: &Path) -> WorkspaceNode {
    let id = dir
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "npm-package".into());
    let manifest_rel = manifest
        .strip_prefix(dir)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| "package.json".into());
    WorkspaceNode {
        id,
        kind: NodeKind::Package,
        root: dir.to_path_buf(),
        path_identity: crate::discovery::blank_identity(),
        source: DiscoverySource {
            provider: "npm-nested".into(),
            manifest: manifest_rel,
        },
        confidence: Confidence::High,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn finds_nested_package_json() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("apps/web")).unwrap();
        fs::write(root.join("apps/web/package.json"), r#"{"name":"web"}"#).unwrap();
        let nodes = discover(root).unwrap();
        assert!(nodes.iter().any(|n| n.id == "web"));
        assert_eq!(nodes[0].source.provider, "npm-nested");
    }
}
