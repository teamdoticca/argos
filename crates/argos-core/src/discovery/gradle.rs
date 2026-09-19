use crate::model::{Confidence, DiscoverySource, NodeKind, WorkspaceNode};
use crate::Result;
use std::path::Path;

/// Android `settings.gradle(.kts)` plus fill-in `build.gradle(.kts)` modules.
pub fn discover(root: &Path) -> Result<Vec<WorkspaceNode>> {
    let mut out = Vec::new();
    for path in super::walk::walk_entries(root) {
        let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        let Some(dir) = path.parent() else {
            continue;
        };
        let lower = name.to_ascii_lowercase();
        if lower == "settings.gradle" || lower == "settings.gradle.kts" {
            out.push(gradle_node(dir, "android-gradle", name));
            continue;
        }
        if lower == "build.gradle" || lower == "build.gradle.kts" {
            out.push(gradle_node(dir, "gradle", name));
        }
    }
    Ok(out)
}

fn gradle_node(dir: &Path, provider: &str, manifest: &str) -> WorkspaceNode {
    WorkspaceNode {
        id: super::walk::dir_id(dir, "gradle-project"),
        kind: NodeKind::Package,
        root: dir.to_path_buf(),
        path_identity: super::blank_identity(),
        source: DiscoverySource {
            provider: provider.into(),
            manifest: manifest.into(),
        },
        confidence: Confidence::High,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn finds_android_settings_and_module() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(
            root.join("settings.gradle.kts"),
            "rootProject.name = \"app\"\n",
        )
        .unwrap();
        fs::create_dir_all(root.join("app")).unwrap();
        fs::write(root.join("app/build.gradle.kts"), "plugins {}\n").unwrap();
        let nodes = discover(root).unwrap();
        assert!(nodes
            .iter()
            .any(|n| n.source.provider == "android-gradle"
                && n.source.manifest == "settings.gradle.kts"));
        assert!(nodes
            .iter()
            .any(|n| n.id == "app" && n.source.provider == "gradle"));
    }
}
