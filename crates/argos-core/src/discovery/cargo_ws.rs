use crate::model::{Confidence, DiscoverySource, NodeKind, WorkspaceNode};
use crate::Result;
use std::path::Path;

pub fn discover(root: &Path) -> Result<Vec<WorkspaceNode>> {
    let path = root.join("Cargo.toml");
    if !path.is_file() {
        return Ok(vec![]);
    }
    let text = std::fs::read_to_string(&path)?;
    let value: toml::Value = text
        .parse()
        .map_err(|e| crate::ArgosError::Parse(format!("Cargo.toml: {e}")))?;
    let Some(members) = value
        .get("workspace")
        .and_then(|w| w.get("members"))
        .and_then(|m| m.as_array())
    else {
        return Ok(vec![]);
    };

    let mut out = Vec::new();
    for m in members {
        let Some(rel) = m.as_str() else { continue };
        // Support simple globs crates/*
        if let Some(parent) = rel.strip_suffix("/*") {
            let base = root.join(parent);
            if let Ok(rd) = std::fs::read_dir(base) {
                for e in rd.flatten() {
                    if e.path().join("Cargo.toml").is_file() {
                        push_member(&mut out, &e.path());
                    }
                }
            }
        } else {
            let dir = root.join(rel);
            if dir.join("Cargo.toml").is_file() {
                push_member(&mut out, &dir);
            }
        }
    }
    Ok(out)
}

fn push_member(out: &mut Vec<WorkspaceNode>, dir: &Path) {
    let id = dir
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "crate".into());
    out.push(WorkspaceNode {
        id,
        kind: NodeKind::Package,
        root: dir.to_path_buf(),
        path_identity: crate::discovery::blank_identity(),
        source: DiscoverySource {
            provider: "cargo".into(),
            manifest: "Cargo.toml".into(),
        },
        confidence: Confidence::High,
    });
}
