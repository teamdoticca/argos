use crate::model::{Confidence, DiscoverySource, NodeKind, WorkspaceNode};
use crate::Result;
use serde::Deserialize;
use std::path::Path;

/// `.workspace` is intentionally minimal. Additional fields are deferred.
#[derive(Debug, Deserialize)]
struct WorkspaceFile {
    imports: Option<Vec<String>>,
}

pub fn discover(root: &Path) -> Result<Vec<WorkspaceNode>> {
    let path = root.join(".workspace");
    if !path.is_file() {
        return Ok(vec![]);
    }
    let text = std::fs::read_to_string(&path)?;
    let file: WorkspaceFile = serde_yaml::from_str(&text)
        .map_err(|e| crate::ArgosError::Parse(format!(".workspace: {e}")))?;
    let Some(imports) = file.imports else {
        return Ok(vec![]);
    };

    let mut out = Vec::new();
    for rel in imports {
        let dir = root.join(&rel);
        let id = dir
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| rel.replace(['/', '\\', '.'], "-"));
        out.push(WorkspaceNode {
            id,
            kind: NodeKind::ImportedFolder,
            root: dir,
            path_identity: crate::discovery::blank_identity(),
            source: DiscoverySource {
                provider: "workspace-import".into(),
                manifest: ".workspace".into(),
            },
            confidence: Confidence::High,
        });
    }
    Ok(out)
}
