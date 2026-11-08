//! Windows filesystem-mode watch integration tests (bounded waits).

use argos_backend::WatchBackend;
use argos_backend_windows::WindowsBackend;
use argos_core::{list_scopes, DomainEvent, FileEventKind, OpenOptions, Workspace};
use std::fs;
use std::thread;
use std::time::{Duration, Instant};

fn fs_opts() -> OpenOptions {
    OpenOptions {
        mode: Some("filesystem".into()),
        ..Default::default()
    }
}

fn wait_for_events<F>(backend: &mut WindowsBackend, timeout: Duration, mut pred: F) -> Vec<DomainEvent>
where
    F: FnMut(&[DomainEvent]) -> bool,
{
    let start = Instant::now();
    let mut collected = Vec::new();
    while start.elapsed() < timeout {
        let batch = backend.poll().unwrap_or_default();
        if !batch.is_empty() {
            collected.extend(batch);
            if pred(&collected) {
                return collected;
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
    collected
}

#[test]
fn create_modify_delete_and_overflow_rescan() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join("Artist")).unwrap();
    let ws = Workspace::open(dir.path(), fs_opts()).unwrap();
    let scopes = list_scopes(&ws.current_snapshot()).to_vec();
    let mut backend = WindowsBackend::new();
    backend.start(&ws, &scopes).unwrap();
    thread::sleep(Duration::from_millis(200));
    let _ = backend.poll();

    let file = dir.path().join("Artist/track.flac");
    fs::write(&file, b"one").unwrap();
    let events = wait_for_events(&mut backend, Duration::from_secs(5), |ev| {
        ev.iter().any(|e| {
            matches!(
                e,
                DomainEvent::File {
                    kind: FileEventKind::FileCreated | FileEventKind::FileModified,
                    ..
                }
            )
        })
    });
    assert!(
        !events.is_empty(),
        "expected create/modify after write, got {events:?}"
    );

    fs::write(&file, b"two").unwrap();
    let _ = wait_for_events(&mut backend, Duration::from_secs(5), |ev| !ev.is_empty());

    fs::remove_file(&file).unwrap();
    let deleted = wait_for_events(&mut backend, Duration::from_secs(5), |ev| !ev.is_empty());
    assert!(!deleted.is_empty(), "expected delete-related events");

    backend.inject_overflow_for_test();
    let overflow = backend.poll().unwrap();
    assert!(
        overflow.iter().any(|e| matches!(
            e,
            DomainEvent::File {
                kind: FileEventKind::ScopeRescan,
                reason: Some(r),
                ..
            } if r == "overflow"
        )),
        "expected overflow rescan, got {overflow:?}"
    );

    backend.stop().unwrap();
    assert!(backend.watched_roots().is_empty());
}

#[test]
fn nested_directory_create_is_observed() {
    let dir = tempfile::tempdir().unwrap();
    let ws = Workspace::open(dir.path(), fs_opts()).unwrap();
    let scopes = list_scopes(&ws.current_snapshot()).to_vec();
    let mut backend = WindowsBackend::new();
    backend.start(&ws, &scopes).unwrap();
    thread::sleep(Duration::from_millis(200));
    let _ = backend.poll();

    fs::create_dir_all(dir.path().join("new/nested")).unwrap();
    fs::write(dir.path().join("new/nested/x.flac"), b"x").unwrap();
    let events = wait_for_events(&mut backend, Duration::from_secs(5), |ev| !ev.is_empty());
    assert!(!events.is_empty(), "expected events for nested create");
    backend.stop().unwrap();
}

#[test]
fn excluded_temp_then_final_rename_visible() {
    let dir = tempfile::tempdir().unwrap();
    let ws = Workspace::open(
        dir.path(),
        OpenOptions {
            mode: Some("filesystem".into()),
            exclude: Some(vec!["**/*.partial".into()]),
            ..Default::default()
        },
    )
    .unwrap();
    let scopes = list_scopes(&ws.current_snapshot()).to_vec();
    let mut backend = WindowsBackend::new();
    backend.start(&ws, &scopes).unwrap();
    thread::sleep(Duration::from_millis(200));
    let _ = backend.poll();

    let partial = dir.path().join("song.flac.partial");
    let final_path = dir.path().join("song.flac");
    fs::write(&partial, b"data").unwrap();
    assert!(!ws.file_event_eligible(&partial));
    fs::rename(&partial, &final_path).unwrap();
    assert!(ws.file_event_eligible(&final_path));

    let events = wait_for_events(&mut backend, Duration::from_secs(5), |ev| !ev.is_empty());
    assert!(
        !events.is_empty(),
        "expected filesystem hints after rename to final path"
    );
    backend.stop().unwrap();
}

#[test]
fn cancellation_clears_watcher_resources() {
    let dir = tempfile::tempdir().unwrap();
    let ws = Workspace::open(dir.path(), fs_opts()).unwrap();
    let scopes = list_scopes(&ws.current_snapshot()).to_vec();
    let mut backend = WindowsBackend::new();
    backend.start(&ws, &scopes).unwrap();
    assert!(!backend.watched_roots().is_empty());
    backend.stop().unwrap();
    assert!(backend.watched_roots().is_empty());
    // Restart is allowed after clean stop.
    backend.start(&ws, &scopes).unwrap();
    backend.stop().unwrap();
}
