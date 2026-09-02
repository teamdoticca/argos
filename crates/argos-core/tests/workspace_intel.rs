//! Integration tests for argos-core workspace intelligence.

use argos_core::{
    compute_delta, discover_topology, explain_path, find_owner, list_scopes, snapshot_validate,
    workspace_health, OpenOptions, Workspace,
};
use std::fs;
use std::path::Path;

fn write(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
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
    write(&root.join("crates/foo/Cargo.toml"), "[package]\nname=\"foo\"\nversion=\"0.1.0\"\n");
    write(&root.join("crates/foo/src/lib.rs"), "");

    let topo = discover_topology(root).unwrap();
    assert!(topo.nodes.iter().any(|n| n.id == "foo"));
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
