use crate::model::{Confidence, DiscoverySource, NodeKind, WorkspaceNode};
use crate::Result;
use std::path::Path;

pub fn discover(root: &Path) -> Result<Vec<WorkspaceNode>> {
    let path = root.join(".gitmodules");
    if !path.is_file() {
        return Ok(vec![]);
    }
    let text = std::fs::read_to_string(&path)?;
    let mut out = Vec::new();
    let mut current_path: Option<String> = None;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            if let Some(p) = current_path.take() {
                push(&mut out, root, &p);
            }
        } else if let Some(rest) = line.strip_prefix("path") {
            let v = rest
                .trim()
                .trim_start_matches('=')
                .trim()
                .trim_matches('"')
                .trim_matches('\'');
            current_path = Some(v.to_string());
        }
    }
    if let Some(p) = current_path {
        push(&mut out, root, &p);
    }
    Ok(out)
}

fn push(out: &mut Vec<WorkspaceNode>, root: &Path, rel: &str) {
    let dir = root.join(rel);
    let id = dir
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| rel.replace('/', "-"));
    out.push(WorkspaceNode {
        id,
        kind: NodeKind::GitSubmodule,
        root: dir,
        path_identity: crate::discovery::blank_identity(),
        source: DiscoverySource {
            provider: "git-submodule".into(),
            manifest: ".gitmodules".into(),
        },
        confidence: Confidence::High,
    });
}
