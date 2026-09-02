use crate::model::{Confidence, DiscoverySource, NodeKind, WorkspaceNode};
use crate::path_identity::{classify_symlink, is_junction_or_reparse, PathIdentity};
use crate::Result;
use std::path::Path;

/// Discover OS-specific artifacts relevant to workspace topology.
/// Windows junctions, Linux mounts, macOS package bundles.
pub fn discover(root: &Path) -> Result<Vec<WorkspaceNode>> {
    let mut out = Vec::new();
    let case_sensitive = crate::path_identity::detect_case_sensitivity(root);

    // Shallow scan of top-level entries for reparse/symlink/mount/bundle markers.
    let Ok(rd) = std::fs::read_dir(root) else {
        return Ok(out);
    };
    for entry in rd.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }

        let mut kind_label: Option<&str> = None;
        if classify_symlink(&path).is_some() {
            kind_label = Some("symlink");
        }
        if is_junction_or_reparse(&path) {
            kind_label = Some(if cfg!(windows) { "junction" } else { "reparse" });
        }
        #[cfg(target_os = "linux")]
        {
            if is_mount_point(&path) {
                kind_label = Some("mount");
            }
        }
        #[cfg(target_os = "macos")]
        {
            if path.extension().and_then(|e| e.to_str()) == Some("app") && path.is_dir() {
                kind_label = Some("bundle");
            }
        }

        let Some(label) = kind_label else {
            continue;
        };
        let id = format!("{name}-{label}");
        out.push(WorkspaceNode {
            id,
            kind: NodeKind::OsArtifact,
            path_identity: PathIdentity::from_path(&path, case_sensitive),
            root: path,
            source: DiscoverySource {
                provider: "os-artifact".into(),
                manifest: label.into(),
            },
            confidence: Confidence::Medium,
        });
    }
    Ok(out)
}

#[cfg(target_os = "linux")]
fn is_mount_point(path: &Path) -> bool {
    // Heuristic: different device id than parent.
    use std::os::unix::fs::MetadataExt;
    let Ok(meta) = std::fs::metadata(path) else {
        return false;
    };
    let Some(parent) = path.parent() else {
        return false;
    };
    let Ok(parent_meta) = std::fs::metadata(parent) else {
        return false;
    };
    meta.dev() != parent_meta.dev()
}
