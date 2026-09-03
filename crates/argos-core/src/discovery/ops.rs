use crate::model::{Confidence, DiscoverySource, NodeKind, WorkspaceNode};
use crate::Result;
use std::path::Path;

/// Dockerfile, Compose, Bicep, and `azure.yaml` as high-signal ops nodes.
pub fn discover(root: &Path) -> Result<Vec<WorkspaceNode>> {
    let mut out = Vec::new();
    for path in super::walk::walk_entries(root) {
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        if is_dockerfile_name(name) {
            out.push(file_node(&path, "dockerfile", name));
            continue;
        }
        if is_compose_name(name) {
            out.push(file_node(&path, "compose", name));
            continue;
        }
        if name.eq_ignore_ascii_case("azure.yaml") || name.eq_ignore_ascii_case("azure.yml") {
            out.push(file_node(&path, "azure-yaml", name));
            continue;
        }
        if path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("bicep"))
        {
            out.push(file_node(&path, "bicep", name));
        }
    }
    Ok(out)
}

fn file_node(path: &Path, provider: &str, manifest: &str) -> WorkspaceNode {
    WorkspaceNode {
        id: format!("{provider}:{}", super::walk::file_id(path, provider)),
        kind: NodeKind::Package,
        root: path.to_path_buf(),
        path_identity: super::blank_identity(),
        source: DiscoverySource {
            provider: provider.into(),
            manifest: manifest.into(),
        },
        confidence: Confidence::High,
    }
}

fn is_dockerfile_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower == "dockerfile" || lower.starts_with("dockerfile.")
}

fn is_compose_name(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "docker-compose.yml"
            | "docker-compose.yaml"
            | "compose.yml"
            | "compose.yaml"
    )
}

/// Best-effort `services.*.build` / `build.context` relative dirs.
pub fn compose_build_contexts(compose_file: &Path) -> Vec<String> {
    let Ok(text) = std::fs::read_to_string(compose_file) else {
        return Vec::new();
    };
    let Some(dir) = compose_file.parent() else {
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut pending_context = false;
    for raw in text.lines() {
        let line = raw.trim();
        if pending_context {
            pending_context = false;
            if let Some(ctx) = line.strip_prefix("context:") {
                push_context(dir, ctx.trim().trim_matches('"').trim_matches('\''), &mut out);
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix("build:") {
            let rest = rest.trim();
            if rest.is_empty() {
                pending_context = true;
            } else if rest != "{" {
                push_context(dir, rest.trim_matches('"').trim_matches('\''), &mut out);
            }
        }
    }
    out
}

fn push_context(dir: &Path, rel: &str, out: &mut Vec<String>) {
    if rel.is_empty() || rel == "." {
        return;
    }
    let path = dir.join(rel);
    if path.is_dir() {
        let rel = rel.replace('\\', "/").trim_start_matches("./").to_string();
        if !out.iter().any(|e| e == &rel) {
            out.push(rel);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn finds_ops_files() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("svc/web")).unwrap();
        fs::write(root.join("svc/Dockerfile"), "FROM alpine\n").unwrap();
        fs::write(
            root.join("svc/docker-compose.yml"),
            "services:\n  web:\n    build: ./web\n",
        )
        .unwrap();
        fs::write(root.join("infra.bicep"), "param x string\n").unwrap();
        fs::write(root.join("azure.yaml"), "name: demo\n").unwrap();
        let nodes = discover(root).unwrap();
        assert!(nodes.iter().any(|n| n.source.provider == "dockerfile"));
        assert!(nodes.iter().any(|n| n.source.provider == "compose"));
        assert!(nodes.iter().any(|n| n.source.provider == "bicep"));
        assert!(nodes.iter().any(|n| n.source.provider == "azure-yaml"));
        let compose = nodes.iter().find(|n| n.source.provider == "compose").unwrap();
        assert!(compose.root.is_file());
        let ctx = compose_build_contexts(&compose.root);
        assert!(ctx.iter().any(|c| c == "web"));
    }
}