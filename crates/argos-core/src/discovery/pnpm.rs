use crate::model::{Confidence, DiscoverySource, NodeKind, WorkspaceNode};
use crate::Result;
use std::path::{Path, PathBuf};

pub fn discover(root: &Path) -> Result<Vec<WorkspaceNode>> {
    let path = root.join("pnpm-workspace.yaml");
    if !path.is_file() {
        return Ok(vec![]);
    }
    let text = std::fs::read_to_string(&path)?;
    let value: serde_yaml::Value = serde_yaml::from_str(&text)
        .map_err(|e| crate::ArgosError::Parse(format!("pnpm-workspace.yaml: {e}")))?;
    let Some(packages) = value.get("packages").and_then(|v| v.as_sequence()) else {
        return Ok(vec![]);
    };

    let mut out = Vec::new();
    for pkg in packages {
        let Some(pattern) = pkg.as_str() else {
            continue;
        };
        for dir in expand_workspace_globs(root, pattern) {
            let id = dir
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| pattern.replace(['*', '/'], "-"));
            out.push(WorkspaceNode {
                id,
                kind: NodeKind::Package,
                root: dir,
                path_identity: crate::discovery::blank_identity(),
                source: DiscoverySource {
                    provider: "pnpm".into(),
                    manifest: "pnpm-workspace.yaml".into(),
                },
                confidence: Confidence::High,
            });
        }
    }
    Ok(out)
}

pub(crate) fn expand_workspace_globs(root: &Path, pattern: &str) -> Vec<PathBuf> {
    // Support simple patterns: "packages/*", "apps/*", exact relative paths.
    if let Some(parent) = pattern.strip_suffix("/*") {
        let base = root.join(parent);
        let mut dirs = Vec::new();
        if let Ok(rd) = std::fs::read_dir(&base) {
            for e in rd.flatten() {
                if e.path().is_dir() {
                    dirs.push(e.path());
                }
            }
        }
        return dirs;
    }
    let p = root.join(pattern);
    if p.is_dir() {
        vec![p]
    } else {
        vec![]
    }
}
