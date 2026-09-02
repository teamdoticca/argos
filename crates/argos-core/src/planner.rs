use crate::ignore_engine::IgnoreEngine;
use crate::model::{NodeKind, WorkspaceNode};
use crate::snapshot::WorkspaceSnapshot;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const KNOWN_DIRS: &[&str] = &[
    "src", "lib", "app", "public", "server", "client", "features", "modules", "packages",
];

const CONFIG_FILES: &[&str] = &[
    "package.json",
    "Cargo.toml",
    "tsconfig.json",
    "pyproject.toml",
    ".workspace",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchScope {
    pub package: String,
    pub root: String,
    pub watch: Vec<String>,
}

pub fn plan_scopes(nodes: &[WorkspaceNode], ignore: &IgnoreEngine) -> Vec<WatchScope> {
    let mut scopes = Vec::new();
    for node in nodes {
        if matches!(node.kind, NodeKind::OsArtifact) {
            continue;
        }
        if matches!(
            node.kind,
            NodeKind::GitSubmodule | NodeKind::ImportedFolder | NodeKind::Package
        ) {
            let watch = match node.source.provider.as_str() {
                "document-root" | "guidance-path" => {
                    plan_document_or_guidance_watch(node, ignore)
                }
                _ => discover_watch_roots(&node.root, ignore),
            };
            if watch.is_empty() {
                continue;
            }
            // For file-targeted guidance nodes, scope.root is the file path.
            scopes.push(WatchScope {
                package: node.id.clone(),
                root: node.root.to_string_lossy().replace('\\', "/"),
                watch,
            });
        }
    }
    scopes
}

fn plan_document_or_guidance_watch(node: &WorkspaceNode, ignore: &IgnoreEngine) -> Vec<String> {
    if node.root.is_file() {
        if ignore.is_ignored(&node.root) {
            return Vec::new();
        }
        return vec![".".into()];
    }
    if !node.root.is_dir() || ignore.is_ignored(&node.root) {
        return Vec::new();
    }
    let manifest = node.source.manifest.as_str();
    if !manifest.is_empty()
        && manifest != "."
        && !manifest.contains('/')
        && !manifest.contains('\\')
        && node.root.join(manifest).is_file()
    {
        return vec![manifest.to_string()];
    }
    vec![".".into()]
}

fn discover_watch_roots(node_root: &Path, ignore: &IgnoreEngine) -> Vec<String> {
    let mut watch = Vec::new();
    for name in KNOWN_DIRS {
        let p = node_root.join(name);
        if p.is_dir() && !ignore.is_ignored(&p) {
            watch.push((*name).to_string());
        }
    }
    for name in CONFIG_FILES {
        let p = node_root.join(name);
        if p.is_file() {
            watch.push((*name).to_string());
        }
    }
    // Include .csproj manifests at package root (.NET).
    if let Ok(rd) = std::fs::read_dir(node_root) {
        for e in rd.flatten() {
            let path = e.path();
            if path
                .extension()
                .and_then(|x| x.to_str())
                .is_some_and(|x| x.eq_ignore_ascii_case("csproj"))
            {
                let name = e.file_name().to_string_lossy().to_string();
                if !watch.iter().any(|w| w == &name) {
                    watch.push(name);
                }
            }
        }
    }
    if let Ok(rd) = std::fs::read_dir(node_root) {
        for e in rd.flatten() {
            let path = e.path();
            if !path.is_dir() {
                continue;
            }
            if ignore.is_ignored(&path) {
                continue;
            }
            let name = e.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }
            if KNOWN_DIRS.contains(&name.as_str()) {
                continue;
            }
            if dir_looks_like_source(&path) && !watch.iter().any(|w| w == &name) {
                watch.push(name);
            }
        }
    }

    // Scoped package-root watch when no conventional dirs found (flat .NET, fallback-root, etc.).
    // Recursive on the *package* root only — never a silent empty plan.
    if watch.is_empty() && node_root.is_dir() && !ignore.is_ignored(node_root) {
        watch.push(".".into());
    }
    watch
}

fn dir_looks_like_source(dir: &Path) -> bool {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return false;
    };
    for e in rd.flatten().take(30) {
        let name = e.file_name().to_string_lossy().to_lowercase();
        if name.ends_with(".ts")
            || name.ends_with(".tsx")
            || name.ends_with(".js")
            || name.ends_with(".rs")
            || name.ends_with(".cs")
            || name.ends_with(".py")
            || name == "src"
            || name == "lib"
        {
            return true;
        }
    }
    false
}

pub fn apply_planner(snapshot: &WorkspaceSnapshot, ignore: &IgnoreEngine) -> Vec<WatchScope> {
    plan_scopes(&snapshot.model.nodes, ignore)
}

pub fn absolute_watch_paths(scope: &WatchScope) -> Vec<PathBuf> {
    let root = PathBuf::from(&scope.root);
    scope
        .watch
        .iter()
        .map(|w| {
            if w == "." {
                root.clone()
            } else {
                root.join(w)
            }
        })
        .filter(|p| p.exists())
        .collect()
}
