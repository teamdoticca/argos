use crate::model::{Confidence, DiscoverySource, NodeKind, WorkspaceNode};
use crate::Result;
use std::path::{Path, PathBuf};

/// Discover Go modules (`go.mod`) and `go.work` members.
pub fn discover(root: &Path) -> Result<Vec<WorkspaceNode>> {
    let mut out = Vec::new();
    for work in find_named(root, "go.work") {
        let Some(work_dir) = work.parent() else {
            continue;
        };
        for member in parse_go_work(&work) {
            let dir = work_dir.join(&member);
            if dir.join("go.mod").is_file() {
                out.push(go_node(&dir, "go-work"));
            }
        }
    }
    for gomod in find_named(root, "go.mod") {
        let Some(dir) = gomod.parent() else {
            continue;
        };
        out.push(go_node(dir, "go-mod"));
    }
    Ok(out)
}

fn go_node(dir: &Path, provider: &str) -> WorkspaceNode {
    WorkspaceNode {
        id: super::walk::dir_id(dir, "go-module"),
        kind: NodeKind::Package,
        root: dir.to_path_buf(),
        path_identity: super::blank_identity(),
        source: DiscoverySource {
            provider: provider.into(),
            manifest: "go.mod".into(),
        },
        confidence: Confidence::High,
    }
}

fn find_named(root: &Path, file_name: &str) -> Vec<PathBuf> {
    super::walk::walk_entries(root)
        .filter(|p| p.file_name().and_then(|s| s.to_str()) == Some(file_name))
        .collect()
}

fn parse_go_work(path: &Path) -> Vec<String> {
    let Ok(text) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut in_use = false;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        if line.starts_with("use") {
            if let Some(rest) = line.strip_prefix("use").map(str::trim) {
                if rest == "(" {
                    in_use = true;
                    continue;
                }
                if rest.starts_with('(') {
                    in_use = true;
                    continue;
                }
                if !rest.is_empty() {
                    out.push(unquote(rest));
                }
            }
            continue;
        }
        if in_use {
            if line.starts_with(')') {
                in_use = false;
                continue;
            }
            out.push(unquote(line));
        }
    }
    out
}

fn unquote(s: &str) -> String {
    s.trim()
        .trim_matches('"')
        .trim_end_matches('/')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn finds_go_mod() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("svc")).unwrap();
        fs::write(root.join("svc/go.mod"), "module example.com/svc\n").unwrap();
        fs::write(root.join("svc/main.go"), "package main\n").unwrap();
        let nodes = discover(root).unwrap();
        assert!(nodes.iter().any(|n| n.id == "svc" && n.source.provider == "go-mod"));
    }

    #[test]
    fn finds_go_work_members() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(
            root.join("go.work"),
            "go 1.22\n\nuse (\n  ./a\n  ./b\n)\n",
        )
        .unwrap();
        fs::create_dir_all(root.join("a")).unwrap();
        fs::create_dir_all(root.join("b")).unwrap();
        fs::write(root.join("a/go.mod"), "module a\n").unwrap();
        fs::write(root.join("b/go.mod"), "module b\n").unwrap();
        let nodes = discover(root).unwrap();
        assert!(nodes.iter().any(|n| n.id == "a" && n.source.provider == "go-work"));
        assert!(nodes.iter().any(|n| n.id == "b"));
    }
}
