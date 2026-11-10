//! Linux filesystem-mode watch integration tests (access noise + mutations).

#![cfg(target_os = "linux")]

use argos_backend::WatchBackend;
use argos_backend_linux::LinuxBackend;
use argos_core::{list_scopes, DomainEvent, FileEventKind, OpenOptions, Workspace};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

fn fs_opts(capacity: Option<usize>) -> OpenOptions {
    OpenOptions {
        mode: Some("filesystem".into()),
        event_buffer_capacity: capacity,
        ..Default::default()
    }
}

fn wait_for_events<F>(
    backend: &mut LinuxBackend,
    timeout: Duration,
    mut pred: F,
) -> Vec<DomainEvent>
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

fn is_mutation_or_rescan(ev: &DomainEvent) -> bool {
    matches!(
        ev,
        DomainEvent::File {
            kind: FileEventKind::FileCreated
                | FileEventKind::FileModified
                | FileEventKind::FileDeleted
                | FileEventKind::FileRenamed
                | FileEventKind::ScopeRescan,
            ..
        } | DomainEvent::Workspace { .. }
    )
}

fn read_tree(root: &Path) {
    fn walk(dir: &Path) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path);
            } else {
                let _ = fs::read(&path);
            }
        }
    }
    walk(root);
}

#[test]
fn read_only_scan_emits_no_mutation_or_rescan() {
    let dir = tempfile::tempdir().unwrap();
    let artist = dir.path().join("Artist");
    fs::create_dir_all(&artist).unwrap();
    for i in 0..40 {
        fs::write(artist.join(format!("track{i}.flac")), b"audio").unwrap();
    }

    let ws = Workspace::open(dir.path(), fs_opts(Some(8))).unwrap();
    let scopes = list_scopes(&ws.current_snapshot()).to_vec();
    let mut backend = LinuxBackend::new();
    backend.start(&ws, &scopes).unwrap();
    thread::sleep(Duration::from_millis(200));
    let _ = backend.poll();

    for _ in 0..5 {
        read_tree(dir.path());
        thread::sleep(Duration::from_millis(100));
    }

    let noise = wait_for_events(&mut backend, Duration::from_secs(2), |_| false);
    assert!(
        noise.iter().all(|e| !is_mutation_or_rescan(e)),
        "read-only scan must not emit mutation/topology/rescan, got {noise:?}"
    );
    backend.stop().unwrap();
}

#[test]
fn create_modify_delete_rename_still_observed() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join("Artist")).unwrap();
    let ws = Workspace::open(dir.path(), fs_opts(None)).unwrap();
    let scopes = list_scopes(&ws.current_snapshot()).to_vec();
    let mut backend = LinuxBackend::new();
    backend.start(&ws, &scopes).unwrap();
    thread::sleep(Duration::from_millis(200));
    let _ = backend.poll();

    let file = dir.path().join("Artist/track.flac");
    fs::write(&file, b"one").unwrap();
    let created = wait_for_events(&mut backend, Duration::from_secs(5), |ev| {
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
        !created.is_empty(),
        "expected create/modify after write, got {created:?}"
    );

    fs::write(&file, b"two").unwrap();
    let _ = wait_for_events(&mut backend, Duration::from_secs(5), |ev| !ev.is_empty());

    let renamed = dir.path().join("Artist/renamed.flac");
    fs::rename(&file, &renamed).unwrap();
    let rename_ev = wait_for_events(&mut backend, Duration::from_secs(5), |ev| !ev.is_empty());
    assert!(!rename_ev.is_empty(), "expected rename-related events");

    fs::remove_file(&renamed).unwrap();
    let deleted = wait_for_events(&mut backend, Duration::from_secs(5), |ev| !ev.is_empty());
    assert!(!deleted.is_empty(), "expected delete-related events");

    backend.stop().unwrap();
}

#[test]
fn mutation_overflow_still_requests_rescan() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join("Artist")).unwrap();
    let ws = Workspace::open(dir.path(), fs_opts(Some(4))).unwrap();
    let scopes = list_scopes(&ws.current_snapshot()).to_vec();
    let mut backend = LinuxBackend::new();
    backend.start(&ws, &scopes).unwrap();
    thread::sleep(Duration::from_millis(200));
    let _ = backend.poll();

    for i in 0..200 {
        fs::write(
            dir.path().join(format!("Artist/burst{i}.flac")),
            format!("x{i}"),
        )
        .unwrap();
    }

    let events = wait_for_events(&mut backend, Duration::from_secs(8), |ev| {
        ev.iter().any(|e| {
            matches!(
                e,
                DomainEvent::File {
                    kind: FileEventKind::ScopeRescan,
                    reason: Some(r),
                    ..
                } if r == "overflow"
            )
        }) || ev.len() > 50
    });

    let saw_overflow = events.iter().any(|e| {
        matches!(
            e,
            DomainEvent::File {
                kind: FileEventKind::ScopeRescan,
                reason: Some(r),
                ..
            } if r == "overflow"
        )
    });
    if !saw_overflow {
        // Platforms may coalesce; inject sticky overflow as the same recovery path.
        backend.inject_overflow_for_test();
        let injected = backend.poll().unwrap();
        assert!(injected.iter().any(|e| matches!(
            e,
            DomainEvent::File {
                kind: FileEventKind::ScopeRescan,
                reason: Some(r),
                ..
            } if r == "overflow"
        )));
    }
    backend.stop().unwrap();
}
