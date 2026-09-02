use crate::discovery::pnpm::expand_workspace_globs;
use crate::model::{Confidence, DiscoverySource, NodeKind, WorkspaceNode};
use crate::Result;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct PackageJson {
    workspaces: Option<WorkspacesField>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum WorkspacesField {
    List(Vec<String>),
    Object { packages: Vec<String> },
}

pub fn discover(root: &Path) -> Result<Vec<WorkspaceNode>> {
    let path = root.join("package.json");
    if !path.is_file() {
        return Ok(vec![]);
    }
    // Prefer pnpm when both exist — already discovered elsewhere; still ok to add unique roots.
    let text = std::fs::read_to_string(&path)?;
    let pkg: PackageJson = serde_json::from_str(&text)
        .map_err(|e| crate::ArgosError::Parse(format!("package.json: {e}")))?;
    let patterns = match pkg.workspaces {
        Some(WorkspacesField::List(v)) => v,
        Some(WorkspacesField::Object { packages }) => packages,
        None => return Ok(vec![]),
    };

    let provider = if root.join("yarn.lock").is_file() {
        "yarn"
    } else {
        "npm"
    };

    let mut out = Vec::new();
    for pattern in patterns {
        for dir in expand_workspace_globs(root, &pattern) {
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
                    provider: provider.into(),
                    manifest: "package.json".into(),
                },
                confidence: Confidence::High,
            });
        }
    }
    Ok(out)
}
