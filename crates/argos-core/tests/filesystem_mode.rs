//! Deterministic filesystem-mode open/coverage/filter/link tests.

use argos_core::{
    list_scopes, OpenOptions, SnapshotState, Workspace, DEFAULT_FILESYSTEM_EVENT_BUFFER,
};
use std::fs;
use std::path::PathBuf;

fn fs_opts() -> OpenOptions {
    OpenOptions {
        mode: Some("filesystem".into()),
        ..Default::default()
    }
}

#[test]
fn plain_nested_tree_without_manifests() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join("Artist/Album")).unwrap();
    fs::write(dir.path().join("Artist/Album/track.flac"), b"flac").unwrap();

    let ws = Workspace::open(dir.path(), fs_opts()).unwrap();
    let snap = ws.current_snapshot();
    assert_eq!(snap.identity.mode, "filesystem");
    assert_eq!(snap.filesystem.symlink_mode, "nofollow");
    assert_eq!(snap.model.nodes.len(), 1);
    assert_eq!(snap.model.nodes[0].id, "filesystem-root");
    assert_eq!(snap.model.nodes[0].source.provider, "filesystem-root");
    let scopes = list_scopes(&snap);
    assert_eq!(scopes.len(), 1);
    assert_eq!(scopes[0].watch, vec![".".to_string()]);
    assert!(snap.planning.ignores.is_empty());
    assert!(snap.planning.coverage.requires_reconciliation);
    assert!(snap
        .planning
        .coverage
        .notes
        .iter()
        .any(|n| n == "startup-gap"));
    assert_eq!(ws.event_buffer_capacity(), DEFAULT_FILESYSTEM_EVENT_BUFFER);
}

#[test]
fn does_not_inherit_software_hard_defaults() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join("node_modules/pkg")).unwrap();
    fs::write(dir.path().join("node_modules/pkg/index.js"), b"x").unwrap();
    fs::write(dir.path().join("song.flac"), b"x").unwrap();
    fs::write(dir.path().join(".gitignore"), "song.flac\n").unwrap();

    let ws = Workspace::open(dir.path(), fs_opts()).unwrap();
    let snap = ws.current_snapshot();
    assert!(snap.planning.ignores.is_empty());
    assert_eq!(snap.planning.scopes[0].watch, vec![".".to_string()]);
}

#[test]
fn include_exclude_semantics_and_traversal() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join("Artist/Album")).unwrap();
    fs::create_dir_all(dir.path().join("staging")).unwrap();
    fs::write(dir.path().join("Artist/Album/a.flac"), b"x").unwrap();
    fs::write(dir.path().join("Artist/Album/a.flac.partial"), b"x").unwrap();
    fs::write(dir.path().join("staging/b.flac"), b"x").unwrap();

    let ws = Workspace::open(
        dir.path(),
        OpenOptions {
            mode: Some("filesystem".into()),
            include: Some(vec!["**/*.flac".into()]),
            exclude: Some(vec!["**/*.partial".into(), "staging/**".into()]),
            ..Default::default()
        },
    )
    .unwrap();

    let snap = ws.current_snapshot();
    assert_eq!(snap.planning.filters.include, vec!["**/*.flac".to_string()]);
    assert!(ws.file_event_eligible(&dir.path().join("Artist/Album/a.flac")));
    assert!(!ws.file_event_eligible(&dir.path().join("Artist/Album/a.flac.partial")));
    assert!(!ws.file_event_eligible(&dir.path().join("staging/b.flac")));
    assert!(ws.file_event_eligible(&dir.path().join("Artist")));
}

#[test]
fn missing_root_is_hard_error_not_empty_success() {
    let missing = PathBuf::from("Z:/this/path/does/not/exist/argos-fs-mode");
    match Workspace::open(&missing, fs_opts()) {
        Ok(_) => panic!("expected open failure for missing root"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains("does not exist") || msg.contains("exist"),
                "unexpected error: {msg}"
            );
        }
    }
}

#[test]
fn workspace_mode_unchanged_default() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("package.json"), r#"{"name":"x"}"#).unwrap();
    let ws = Workspace::open(dir.path(), OpenOptions::default()).unwrap();
    let snap = ws.current_snapshot();
    assert_eq!(snap.identity.mode, "workspace");
    assert_eq!(snap.filesystem.symlink_mode, "follow");
    assert!(!snap.planning.ignores.is_empty());
}

#[cfg(unix)]
#[test]
fn nofollow_skips_links_follow_confined_keeps_in_root() {
    let dir = tempfile::tempdir().unwrap();
    let inside = dir.path().join("inside");
    let outside = tempfile::tempdir().unwrap();
    fs::create_dir_all(&inside).unwrap();
    fs::write(inside.join("a.flac"), b"x").unwrap();
    fs::write(outside.path().join("out.flac"), b"y").unwrap();
    std::os::unix::fs::symlink(&inside, dir.path().join("link-in")).unwrap();
    std::os::unix::fs::symlink(outside.path(), dir.path().join("link-out")).unwrap();

    let nofollow = Workspace::open(dir.path(), fs_opts()).unwrap();
    let cov = &nofollow.current_snapshot().planning.coverage;
    assert!(cov
        .skipped_links
        .iter()
        .any(|p| p.contains("link-in") || p.contains("link-out")));

    let confined = Workspace::open(
        dir.path(),
        OpenOptions {
            mode: Some("filesystem".into()),
            symlink_mode: Some("followConfined".into()),
            ..Default::default()
        },
    )
    .unwrap();
    let cov = &confined.current_snapshot().planning.coverage;
    assert!(cov
        .skipped_outside_root
        .iter()
        .any(|p| p.contains("link-out")));
}

#[cfg(unix)]
#[test]
fn follow_confined_detects_cycles() {
    let dir = tempfile::tempdir().unwrap();
    let a = dir.path().join("a");
    fs::create_dir_all(&a).unwrap();
    std::os::unix::fs::symlink(&a, a.join("loop")).unwrap();
    let ws = Workspace::open(
        dir.path(),
        OpenOptions {
            mode: Some("filesystem".into()),
            symlink_mode: Some("followConfined".into()),
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(ws.current_snapshot().model.nodes[0].id, "filesystem-root");
}

#[test]
fn root_disappearance_marks_degraded() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join("lib")).unwrap();
    let ws = Workspace::open(dir.path(), fs_opts()).unwrap();
    let snap = ws
        .mark_degraded_root_unavailable("root-unavailable")
        .unwrap();
    assert_eq!(snap.identity.state, SnapshotState::Degraded);
    assert!(snap.planning.coverage.requires_reconciliation);
}

#[test]
fn recovering_sets_reconciliation_flag() {
    let dir = tempfile::tempdir().unwrap();
    let ws = Workspace::open(dir.path(), fs_opts()).unwrap();
    let snap = ws.mark_recovering().unwrap();
    assert_eq!(snap.identity.state, SnapshotState::Recovering);
    assert!(snap.planning.coverage.requires_reconciliation);
}
