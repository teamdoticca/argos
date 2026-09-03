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
    "Pipfile",
    "requirements.txt",
    "composer.json",
    "go.mod",
    "go.work",
    "pom.xml",
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
                "document-root" | "guidance-path" | "dockerfile" | "bicep" | "azure-yaml" => {
                    plan_document_or_guidance_watch(node, ignore)
                }
                "compose" => plan_compose_watch(node, ignore),
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

fn plan_compose_watch(node: &WorkspaceNode, ignore: &IgnoreEngine) -> Vec<String> {
    if ignore.is_ignored(&node.root) {
        return Vec::new();
    }
    let compose_file = if node.root.is_file() {
        node.root.clone()
    } else {
        let manifest = node.source.manifest.as_str();
        let candidate = node.root.join(manifest);
        if !candidate.is_file() {
            return vec![".".into()];
        }
        candidate
    };
    let mut watch = if node.root.is_file() {
        vec![".".into()]
    } else {
        vec![node.source.manifest.clone()]
    };
    let extra = crate::discovery::ops_compose_build_contexts(&compose_file);
    let parent = compose_file.parent();
    for dir in extra {
        let Some(parent) = parent else {
            break;
        };
        let abs = parent.join(&dir);
        if !abs.is_dir() || ignore.is_ignored(&abs) {
            continue;
        }
        let rel = if node.root.is_file() {
            format!("../{}", dir.trim_start_matches("./"))
        } else {
            dir
        };
        if !watch.iter().any(|w| w == &rel) {
            watch.push(rel);
        }
    }
    watch
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

    // Flat packages (SDK-style .NET, Electron, etc.) often keep sources at the package root.
    // A lone .csproj / package.json must not block the package-root fallback — otherwise
    // Focused consumers miss Program.cs / main.js next to the manifest.
    let has_package_root_watch = watch.iter().any(|w| w == ".");
    let only_manifests = !watch.is_empty() && watch.iter().all(|w| is_manifest_watch_entry(w));
    let root_has_sources = package_root_has_source_files(node_root);
    if !has_package_root_watch
        && (watch.is_empty() || only_manifests || root_has_sources)
        && node_root.is_dir()
        && !ignore.is_ignored(node_root)
    {
        watch.insert(0, ".".into());
    }
    watch
}

fn is_manifest_watch_entry(name: &str) -> bool {
    if CONFIG_FILES.iter().any(|c| *c == name) {
        return true;
    }
    let lower = name.to_ascii_lowercase();
    lower.ends_with(".csproj") || lower == "package-lock.json" || lower == "yarn.lock"
}

fn package_root_has_source_files(node_root: &Path) -> bool {
    let Ok(rd) = std::fs::read_dir(node_root) else {
        return false;
    };
    for e in rd.flatten().take(80) {
        let path = e.path();
        if !path.is_file() {
            continue;
        }
        let name = e.file_name().to_string_lossy().to_lowercase();
        if name.ends_with(".cs")
            || name.ends_with(".ts")
            || name.ends_with(".tsx")
            || name.ends_with(".js")
            || name.ends_with(".jsx")
            || name.ends_with(".mjs")
            || name.ends_with(".cjs")
            || name.ends_with(".rs")
            || name.ends_with(".py")
            || name.ends_with(".php")
            || name.ends_with(".go")
            || name.ends_with(".java")
            || name.ends_with(".kt")
            || name.ends_with(".kts")
            || name.ends_with(".html")
            || name.ends_with(".vue")
            || name.ends_with(".svelte")
        {
            return true;
        }
    }
    false
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
            || name.ends_with(".php")
            || name.ends_with(".go")
            || name.ends_with(".java")
            || name.ends_with(".kt")
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::blank_identity;
    use crate::model::{Confidence, DiscoverySource, NodeKind, WorkspaceNode};
    use std::fs;

    fn package_node(id: &str, root: PathBuf, provider: &str, manifest: &str) -> WorkspaceNode {
        WorkspaceNode {
            id: id.into(),
            kind: NodeKind::Package,
            root,
            path_identity: blank_identity(),
            source: DiscoverySource {
                provider: provider.into(),
                manifest: manifest.into(),
            },
            confidence: Confidence::High,
        }
    }

    #[test]
    fn flat_dotnet_package_watches_package_root() {
        let dir = tempfile::tempdir().unwrap();
        let pkg = dir.path().join("Mnemon.Tray");
        fs::create_dir_all(&pkg).unwrap();
        fs::write(pkg.join("Mnemon.Tray.csproj"), "<Project />").unwrap();
        fs::write(pkg.join("Program.cs"), "//").unwrap();
        fs::write(pkg.join("DaemonHealth.cs"), "//").unwrap();

        let ignore = IgnoreEngine::build(dir.path(), true).unwrap();
        let scopes = plan_scopes(
            &[package_node(
                "Mnemon.Tray",
                pkg.clone(),
                "dotnet-csproj",
                "Mnemon.Tray.csproj",
            )],
            &ignore,
        );
        assert_eq!(scopes.len(), 1);
        assert!(
            scopes[0].watch.iter().any(|w| w == "."),
            "expected '.' for flat .NET package, got {:?}",
            scopes[0].watch
        );
    }

    #[test]
    fn dotnet_with_subdirs_and_root_cs_still_watches_package_root() {
        let dir = tempfile::tempdir().unwrap();
        let pkg = dir.path().join("Mnemon.Api");
        fs::create_dir_all(pkg.join("Endpoints")).unwrap();
        fs::write(pkg.join("Endpoints/Health.cs"), "//").unwrap();
        fs::write(pkg.join("Mnemon.Api.csproj"), "<Project />").unwrap();
        fs::write(pkg.join("Program.cs"), "//").unwrap();

        let ignore = IgnoreEngine::build(dir.path(), true).unwrap();
        let scopes = plan_scopes(
            &[package_node(
                "Mnemon.Api",
                pkg,
                "dotnet-csproj",
                "Mnemon.Api.csproj",
            )],
            &ignore,
        );
        assert_eq!(scopes.len(), 1);
        assert!(
            scopes[0].watch.iter().any(|w| w == "."),
            "root Program.cs must be covered, got {:?}",
            scopes[0].watch
        );
    }

    #[test]
    fn flat_electron_package_watches_package_root() {
        let dir = tempfile::tempdir().unwrap();
        let pkg = dir.path().join("mnemon-electron");
        fs::create_dir_all(&pkg).unwrap();
        fs::write(pkg.join("package.json"), r#"{"name":"mnemon-electron"}"#).unwrap();
        fs::write(pkg.join("main.js"), "//").unwrap();
        fs::write(pkg.join("loading.html"), "<html></html>").unwrap();

        let ignore = IgnoreEngine::build(dir.path(), true).unwrap();
        let scopes = plan_scopes(
            &[package_node(
                "mnemon-electron",
                pkg,
                "npm-nested",
                "package.json",
            )],
            &ignore,
        );
        assert_eq!(scopes.len(), 1);
        assert!(
            scopes[0].watch.iter().any(|w| w == "."),
            "expected '.' for flat Electron package, got {:?}",
            scopes[0].watch
        );
    }

    #[test]
    fn conventional_src_only_package_keeps_src_without_forced_dot_when_no_root_sources() {
        let dir = tempfile::tempdir().unwrap();
        let pkg = dir.path().join("libpkg");
        fs::create_dir_all(pkg.join("src")).unwrap();
        fs::write(pkg.join("src/index.ts"), "//").unwrap();
        fs::write(pkg.join("package.json"), r#"{"name":"libpkg"}"#).unwrap();

        let ignore = IgnoreEngine::build(dir.path(), true).unwrap();
        let scopes = plan_scopes(
            &[package_node("libpkg", pkg, "npm-nested", "package.json")],
            &ignore,
        );
        assert_eq!(scopes.len(), 1);
        assert!(scopes[0].watch.iter().any(|w| w == "src"));
        assert!(
            !scopes[0].watch.iter().any(|w| w == "."),
            "no root sources → should not force '.', got {:?}",
            scopes[0].watch
        );
    }

    #[test]
    fn dockerfile_file_node_watches_dot() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("Dockerfile");
        fs::write(&file, "FROM alpine\n").unwrap();
        let ignore = IgnoreEngine::build(dir.path(), true).unwrap();
        let scopes = plan_scopes(
            &[package_node("df", file, "dockerfile", "Dockerfile")],
            &ignore,
        );
        assert_eq!(scopes.len(), 1);
        assert_eq!(scopes[0].watch, vec![".".to_string()]);
    }

    #[test]
    fn compose_file_watches_dot_and_build_context() {
        let dir = tempfile::tempdir().unwrap();
        let svc = dir.path().join("svc");
        fs::create_dir_all(svc.join("web")).unwrap();
        let compose = svc.join("docker-compose.yml");
        fs::write(
            &compose,
            "services:\n  web:\n    build: ./web\n",
        )
        .unwrap();
        let ignore = IgnoreEngine::build(dir.path(), true).unwrap();
        let scopes = plan_scopes(
            &[package_node(
                "compose",
                compose,
                "compose",
                "docker-compose.yml",
            )],
            &ignore,
        );
        assert_eq!(scopes.len(), 1);
        assert!(scopes[0].watch.iter().any(|w| w == "."));
        assert!(
            scopes[0].watch.iter().any(|w| w == "../web"),
            "expected ../web build context, got {:?}",
            scopes[0].watch
        );
    }
}
