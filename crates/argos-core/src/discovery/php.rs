use crate::model::{Confidence, DiscoverySource, NodeKind, WorkspaceNode};
use crate::Result;
use std::path::Path;

/// Discover PHP Composer packages (`composer.json` outside `vendor/`).
pub fn discover(root: &Path) -> Result<Vec<WorkspaceNode>> {
    let mut out = Vec::new();
    for path in super::walk::walk_entries(root) {
        if path.file_name().and_then(|s| s.to_str()) != Some("composer.json") {
            continue;
        }
        let Some(dir) = path.parent() else {
            continue;
        };
        out.push(WorkspaceNode {
            id: super::walk::dir_id(dir, "php-package"),
            kind: NodeKind::Package,
            root: dir.to_path_buf(),
            path_identity: super::blank_identity(),
            source: DiscoverySource {
                provider: "php-composer".into(),
                manifest: "composer.json".into(),
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
    fn finds_composer_json_skips_vendor() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("app")).unwrap();
        fs::write(root.join("app/composer.json"), r#"{"name":"app/pkg"}"#).unwrap();
        fs::create_dir_all(root.join("app/vendor/slim")).unwrap();
        fs::write(
            root.join("app/vendor/slim/composer.json"),
            r#"{"name":"slim/slim"}"#,
        )
        .unwrap();
        let nodes = discover(root).unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].id, "app");
        assert_eq!(nodes[0].source.provider, "php-composer");
    }
}
