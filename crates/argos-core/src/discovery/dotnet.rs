use crate::model::{Confidence, DiscoverySource, NodeKind, WorkspaceNode};
use crate::Result;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Discover .NET projects (`.csproj`) and solution members as package nodes.
pub fn discover(root: &Path) -> Result<Vec<WorkspaceNode>> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();

    // Prefer .sln Project entries when present (higher signal), then fill with loose .csproj.
    for sln in find_files(root, "sln")? {
        for proj in parse_sln_projects(&sln)? {
            let dir = proj
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| proj.clone());
            let key = dir.to_string_lossy().to_lowercase();
            if !seen.insert(key) {
                continue;
            }
            out.push(csproj_node(&dir, &proj, "dotnet-sln"));
        }
    }

    for csproj in find_files(root, "csproj")? {
        let Some(dir) = csproj.parent() else {
            continue;
        };
        let key = dir.to_string_lossy().to_lowercase();
        if !seen.insert(key) {
            continue;
        }
        out.push(csproj_node(dir, &csproj, "dotnet-csproj"));
    }

    Ok(out)
}

fn csproj_node(dir: &Path, manifest: &Path, provider: &str) -> WorkspaceNode {
    let id = dir
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .or_else(|| {
            manifest
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| "dotnet-project".into());
    let manifest_name = manifest
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "project.csproj".into());
    WorkspaceNode {
        id,
        kind: NodeKind::Package,
        root: dir.to_path_buf(),
        path_identity: crate::discovery::blank_identity(),
        source: DiscoverySource {
            provider: provider.into(),
            manifest: manifest_name,
        },
        confidence: Confidence::High,
    }
}

fn find_files(root: &Path, extension: &str) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for path in super::walk::walk_entries(root) {
        if path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case(extension))
        {
            out.push(path);
        }
    }
    Ok(out)
}

fn parse_sln_projects(sln: &Path) -> Result<Vec<PathBuf>> {
    let text = std::fs::read_to_string(sln)?;
    let Some(sln_dir) = sln.parent() else {
        return Ok(vec![]);
    };
    let mut out = Vec::new();
    for line in text.lines() {
        // Project("{GUID}") = "Name", "rel\path.csproj", "{GUID}"
        let line = line.trim();
        if !line.starts_with("Project(") {
            continue;
        }
        let Some(rel) = extract_sln_project_path(line) else {
            continue;
        };
        if !rel.to_ascii_lowercase().ends_with(".csproj") {
            continue;
        }
        // Normalize slashes so .sln paths resolve on Windows and Unix.
        let path = sln_dir.join(rel.replace('\\', "/"));
        if path.is_file() {
            out.push(path);
        }
    }
    Ok(out)
}

fn extract_sln_project_path(line: &str) -> Option<String> {
    // Split on quotes: ... = "Name", "path.csproj", "{guid}"
    let parts: Vec<&str> = line.split('"').collect();
    // Expected odd-index strings: guid, Name, path, guid
    if parts.len() >= 6 {
        Some(parts[5].replace('\\', "/"))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn finds_csproj() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("src/App")).unwrap();
        fs::write(
            root.join("src/App/App.csproj"),
            r#"<Project Sdk="Microsoft.NET.Sdk"></Project>"#,
        )
        .unwrap();
        let nodes = discover(root).unwrap();
        assert!(nodes.iter().any(|n| n.id == "App"));
        assert_eq!(nodes[0].source.provider, "dotnet-csproj");
    }

    #[test]
    fn parses_sln_project_line() {
        let line = r#"Project("{FAE04EC0-301F-11D3-BF4B-00C04F79EFBC}") = "App", "src\App\App.csproj", "{AAAAAAAA-AAAA-AAAA-AAAA-AAAAAAAAAAAA}""#;
        let rel = extract_sln_project_path(line).unwrap();
        assert!(rel.ends_with("App.csproj"));
    }
}
