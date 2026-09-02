//! Integration tests for argos-core workspace intelligence.

use argos_core::{
    compute_delta, discover_topology, explain_path, find_owner, list_scopes, snapshot_validate,
    workspace_health, OpenOptions, Workspace,
};
use std::fs;
use std::path::{Path, PathBuf};

fn write(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(name)
}

#[test]
fn discovers_pnpm_workspace() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    write(
        &root.join("pnpm-workspace.yaml"),
        "packages:\n  - 'packages/*'\n",
    );
    write(&root.join("packages/a/package.json"), r#"{"name":"a"}"#);
    write(&root.join("packages/a/src/index.ts"), "export {}");
    write(&root.join("packages/b/package.json"), r#"{"name":"b"}"#);
    write(&root.join("packages/b/src/index.ts"), "export {}");

    let topo = discover_topology(root).unwrap();
    assert!(topo.nodes.iter().any(|n| n.id == "a"));
    assert!(topo.nodes.iter().any(|n| n.id == "b"));
    assert!(topo
        .nodes
        .iter()
        .any(|n| n.source.provider == "pnpm" && n.confidence == argos_core::Confidence::High));
}

#[test]
fn discovers_cargo_workspace() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    write(
        &root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/*\"]\n",
    );
    write(
        &root.join("crates/foo/Cargo.toml"),
        "[package]\nname=\"foo\"\nversion=\"0.1.0\"\n",
    );
    write(&root.join("crates/foo/src/lib.rs"), "");

    let topo = discover_topology(root).unwrap();
    assert!(topo.nodes.iter().any(|n| n.id == "foo"));
}

#[test]
fn discovers_nested_npm_without_workspace_root() {
    let root = fixture("nested-npm");
    assert!(root.is_dir(), "missing fixture {}", root.display());
    let topo = discover_topology(&root).unwrap();
    assert!(
        topo.nodes
            .iter()
            .any(|n| n.id == "web" && n.source.provider == "npm-nested"),
        "expected nested web package, got {:?}",
        topo.nodes
            .iter()
            .map(|n| (&n.id, &n.source.provider))
            .collect::<Vec<_>>()
    );
    let ws = Workspace::open(&root, OpenOptions::default()).unwrap();
    let snap = ws.current_snapshot();
    assert!(
        !list_scopes(&snap).is_empty(),
        "nested-npm must plan non-empty scopes"
    );
}

#[test]
fn discovers_dotnet_sln_and_csproj() {
    let root = fixture("dotnet-sln");
    assert!(root.is_dir(), "missing fixture {}", root.display());
    let topo = discover_topology(&root).unwrap();
    assert!(
        topo.nodes.iter().any(|n| n.id == "App"),
        "expected App csproj node, got {:?}",
        topo.nodes.iter().map(|n| &n.id).collect::<Vec<_>>()
    );
    assert!(topo.nodes.iter().any(|n| {
        n.source.provider == "dotnet-sln" || n.source.provider == "dotnet-csproj"
    }));
    let ws = Workspace::open(&root, OpenOptions::default()).unwrap();
    let snap = ws.current_snapshot();
    let scopes = list_scopes(&snap);
    assert!(!scopes.is_empty(), "dotnet-sln must plan non-empty scopes");
}

#[test]
fn fixture_small_pnpm_still_discovers() {
    let root = fixture("small-pnpm");
    assert!(root.is_dir(), "missing fixture {}", root.display());
    let topo = discover_topology(&root).unwrap();
    assert!(topo
        .nodes
        .iter()
        .any(|n| n.id == "app" || n.root.ends_with("app")));
    assert!(topo
        .nodes
        .iter()
        .any(|n| n.id == "lib" || n.root.ends_with("lib")));
    let ws = Workspace::open(&root, OpenOptions::default()).unwrap();
    let snap = ws.current_snapshot();
    assert!(!list_scopes(&snap).is_empty());
}

#[test]
fn workspace_imports_only() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let shared = dir.path().join("shared");
    fs::create_dir_all(&shared).unwrap();
    write(&root.join(".workspace"), "imports:\n  - shared\n");
    write(&shared.join("src/lib.rs"), "");

    let topo = discover_topology(root).unwrap();
    let imported = topo
        .nodes
        .iter()
        .find(|n| n.source.provider == "workspace-import")
        .expect("import node");
    assert_eq!(imported.kind, argos_core::NodeKind::ImportedFolder);
}

#[test]
fn open_builds_snapshot_with_scopes_and_versions() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    write(
        &root.join("pnpm-workspace.yaml"),
        "packages:\n  - 'packages/*'\n",
    );
    write(&root.join("packages/app/package.json"), r#"{"name":"app"}"#);
    write(&root.join("packages/app/src/main.ts"), "console.log(1)");
    write(&root.join(".gitignore"), "node_modules\n");

    let ws = Workspace::open(root, OpenOptions::default()).unwrap();
    let snap = ws.current_snapshot();
    assert_eq!(snap.identity.snapshot_version, 1);
    assert_eq!(snap.identity.content_version, 1);
    assert!(!list_scopes(&snap).is_empty());
    let report = snapshot_validate(&snap);
    assert!(report.valid);
    let health = workspace_health(&snap);
    assert_eq!(health.status, "healthy");
}

#[test]
fn rebuild_without_change_keeps_content_version() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    write(&root.join("package.json"), r#"{"name":"solo"}"#);
    write(&root.join("src/index.ts"), "export {}");

    let ws = Workspace::open(root, OpenOptions::default()).unwrap();
    let a = ws.current_snapshot();
    let b = ws.rebuild_snapshot().unwrap();
    assert_eq!(b.identity.snapshot_version, a.identity.snapshot_version + 1);
    assert_eq!(b.identity.content_version, a.identity.content_version);
    let delta = compute_delta(&a, &b).unwrap();
    assert!(!delta.content_changed);
}

#[test]
fn ignore_engine_hard_defaults() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    write(&root.join("package.json"), r#"{"name":"x"}"#);
    fs::create_dir_all(root.join("node_modules/pkg")).unwrap();
    write(&root.join("node_modules/pkg/index.js"), "");
    write(&root.join("src/app.ts"), "");

    let ws = Workspace::open(root, OpenOptions::default()).unwrap();
    assert!(ws.ignore().is_ignored(&root.join("node_modules/pkg/index.js")));
    let snap = ws.current_snapshot();
    let explained = explain_path(
        &snap,
        &root.join("src/app.ts"),
        ws.ignore().is_ignored(&root.join("src/app.ts")),
        None,
    );
    assert!(explained.exists);
    assert!(!explained.ignored);
}

#[test]
fn find_owner_prefers_deepest_node() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    write(
        &root.join("pnpm-workspace.yaml"),
        "packages:\n  - 'packages/*'\n",
    );
    write(&root.join("packages/lib/package.json"), r#"{"name":"lib"}"#);
    write(&root.join("packages/lib/src/a.ts"), "");

    let ws = Workspace::open(root, OpenOptions::default()).unwrap();
    let snap = ws.current_snapshot();
    let owner = find_owner(&snap, &root.join("packages/lib/src/a.ts")).unwrap();
    assert_eq!(owner.node_id, "lib");
}

#[test]
fn empty_tree_gets_explicit_fallback_root() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    write(&root.join("README.md"), "empty-ish");
    let topo = discover_topology(root).unwrap();
    assert!(topo
        .nodes
        .iter()
        .any(|n| n.source.provider == "fallback-root"));
    let ws = Workspace::open(root, OpenOptions::default()).unwrap();
    let snap = ws.current_snapshot();
    assert!(
        !list_scopes(&snap).is_empty(),
        "fallback-root must still yield a watch plan"
    );
}

#[test]
fn discovers_document_root_handbook_fixture() {
    let root = fixture("docs-handbook");
    assert!(root.is_dir(), "missing fixture {}", root.display());
    let topo = discover_topology(&root).unwrap();
    assert!(
        topo.nodes.iter().any(|n| {
            n.source.provider == "document-root" && n.root.ends_with("handbook")
        }),
        "expected handbook document-root, got {:?}",
        topo.nodes
            .iter()
            .map(|n| (&n.id, &n.source.provider, &n.root))
            .collect::<Vec<_>>()
    );
    let ws = Workspace::open(&root, OpenOptions::default()).unwrap();
    let snap = ws.current_snapshot();
    let scopes = list_scopes(&snap);
    assert!(
        scopes.iter().any(|s| s.package.starts_with("docs:")),
        "expected docs scope, got {:?}",
        scopes
    );
}

#[test]
fn discovers_guidance_seeded_odd_docs_and_rules() {
    let root = fixture("docs-guidance-odd");
    assert!(root.is_dir(), "missing fixture {}", root.display());
    let topo = discover_topology(&root).unwrap();
    assert!(
        topo.nodes.iter().any(|n| {
            n.source.provider == "document-root" && n.root.ends_with("knowledge-base")
        }),
        "expected knowledge-base document-root"
    );
    assert!(
        topo.nodes.iter().any(|n| {
            n.source.provider == "guidance-path"
                && (n.root.ends_with("rules") || n.root.ends_with("AGENTS.md"))
        }),
        "expected guidance-path for rules or AGENTS.md, got {:?}",
        topo.nodes
            .iter()
            .map(|n| (&n.id, &n.source.provider))
            .collect::<Vec<_>>()
    );
    let ws = Workspace::open(&root, OpenOptions::default()).unwrap();
    assert!(!list_scopes(&ws.current_snapshot()).is_empty());
}

#[test]
fn argos_self_repo_includes_docs_and_guidance_scopes() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let root = root.canonicalize().unwrap_or(root);
    if !root.join("AGENTS.md").is_file() || !root.join("docs/roadmap.md").is_file() {
        return; // not the Argos checkout
    }
    let topo = discover_topology(&root).unwrap();
    assert!(
        topo.nodes
            .iter()
            .any(|n| n.source.provider == "document-root"),
        "Argos repo should discover a document-root"
    );
    assert!(
        topo.nodes.iter().any(|n| n.source.provider == "guidance-path"),
        "Argos repo should discover guidance-path nodes"
    );
    let ws = Workspace::open(&root, OpenOptions::default()).unwrap();
    let snap = ws.current_snapshot();
    let scopes = list_scopes(&snap);
    assert!(
        scopes.iter().any(|s| {
            s.root.contains("docs")
                || s.package.starts_with("docs:")
                || s.package.starts_with("guidance:")
        }),
        "expected docs/guidance scopes, got {} scopes",
        scopes.len()
    );
}
