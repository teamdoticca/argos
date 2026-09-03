use crate::model::{Confidence, DiscoverySource, NodeKind, WorkspaceNode};
use crate::Result;
use std::path::Path;

/// Discover Maven modules (`pom.xml`).
pub fn discover(root: &Path) -> Result<Vec<WorkspaceNode>> {
    let mut out = Vec::new();
    for path in super::walk::walk_entries(root) {
        if path.file_name().and_then(|s| s.to_str()) != Some("pom.xml") {
            continue;
        }
        let Some(dir) = path.parent() else {
            continue;
        };
        out.push(WorkspaceNode {
            id: super::walk::dir_id(dir, "maven-module"),
            kind: NodeKind::Package,
            root: dir.to_path_buf(),
            path_identity: super::blank_identity(),
            source: DiscoverySource {
                provider: "maven".into(),
                manifest: "pom.xml".into(),
            },
            confidence: Confidence::High,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn finds_pom_skips_target() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(root.join("pom.xml"), "<project></project>\n").unwrap();
        fs::create_dir_all(root.join("target")).unwrap();
        fs::write(root.join("target/pom.xml"), "<project></project>\n").unwrap();
        let nodes = discover(root).unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].source.provider, "maven");
    }
}
