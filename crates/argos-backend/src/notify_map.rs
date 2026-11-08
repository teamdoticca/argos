//! Shared notify → DomainEvent mapping and bounded receive queue.

use argos_core::{DiscoveryMode, DomainEvent, FileEventKind, WorkspaceEventKind};
use notify::event::{ModifyKind, RenameMode};
use notify::{Config, Event, EventHandler, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TryRecvError, TrySendError};
use std::sync::Arc;

const TOPOLOGY_MANIFESTS: &[&str] = &[
    "pnpm-workspace.yaml",
    "package.json",
    "Cargo.toml",
    ".workspace",
    ".gitmodules",
];

/// Bounded notify event queue. Overflow sets a sticky flag drained as `scopeRescan`.
pub struct BoundedWatchQueue {
    rx: Receiver<notify::Result<Event>>,
    overflow: Arc<AtomicBool>,
    disconnected: AtomicBool,
}

struct OverflowHandler {
    tx: SyncSender<notify::Result<Event>>,
    overflow: Arc<AtomicBool>,
}

impl EventHandler for OverflowHandler {
    fn handle_event(&mut self, event: notify::Result<Event>) {
        match self.tx.try_send(event) {
            Ok(()) => {}
            Err(TrySendError::Full(_)) | Err(TrySendError::Disconnected(_)) => {
                self.overflow.store(true, Ordering::SeqCst);
            }
        }
    }
}

pub fn start_bounded_watcher(
    capacity: usize,
    roots: &[(PathBuf, RecursiveMode)],
) -> argos_core::Result<(RecommendedWatcher, BoundedWatchQueue)> {
    let capacity = capacity.max(1);
    let (tx, rx) = sync_channel(capacity);
    let overflow = Arc::new(AtomicBool::new(false));
    let handler = OverflowHandler {
        tx,
        overflow: overflow.clone(),
    };
    let mut watcher = RecommendedWatcher::new(handler, Config::default())
        .map_err(|e| argos_core::ArgosError::Message(format!("watcher: {e}")))?;
    for (path, mode) in roots {
        watcher
            .watch(path, *mode)
            .map_err(|e| argos_core::ArgosError::Message(format!("watch: {e}")))?;
    }
    Ok((
        watcher,
        BoundedWatchQueue {
            rx,
            overflow,
            disconnected: AtomicBool::new(false),
        },
    ))
}

impl BoundedWatchQueue {
    /// Test/helper: inject overflow without a real notify storm.
    pub fn force_overflow(&self) {
        self.overflow.store(true, Ordering::SeqCst);
    }

    pub fn drain(
        &self,
        mode: DiscoveryMode,
        root: &Path,
        filter: Option<&dyn Fn(&Path) -> bool>,
    ) -> argos_core::Result<Vec<DomainEvent>> {
        let mut events = Vec::new();
        if !root.exists() {
            events.push(DomainEvent::file_with_reason(
                FileEventKind::ScopeRescan,
                root.to_string_lossy().replace('\\', "/"),
                "root-unavailable",
            ));
            return Ok(events);
        }

        loop {
            match self.rx.try_recv() {
                Ok(Ok(ev)) => {
                    events.extend(map_notify_event(ev, mode, filter));
                }
                Ok(Err(e)) => {
                    let msg = e.to_string().to_lowercase();
                    if is_overflow_message(&msg) {
                        self.overflow.store(true, Ordering::SeqCst);
                    } else {
                        events.push(DomainEvent::file_with_reason(
                            FileEventKind::ScopeRescan,
                            String::new(),
                            "watch-failed",
                        ));
                        return Ok(events);
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    self.disconnected.store(true, Ordering::SeqCst);
                    events.push(DomainEvent::file_with_reason(
                        FileEventKind::ScopeRescan,
                        String::new(),
                        "watch-failed",
                    ));
                    break;
                }
            }
        }

        if self.overflow.swap(false, Ordering::SeqCst) {
            events.push(DomainEvent::file_with_reason(
                FileEventKind::ScopeRescan,
                String::new(),
                "overflow",
            ));
        }
        Ok(events)
    }
}

fn is_overflow_message(msg: &str) -> bool {
    msg.contains("overflow")
        || msg.contains("q_overflow")
        || msg.contains("buffer")
        || msg.contains("notify_enum")
        || msg.contains("rescan")
        || msg.contains("invalidate")
}

pub fn map_notify_event(
    ev: Event,
    mode: DiscoveryMode,
    filter: Option<&dyn Fn(&Path) -> bool>,
) -> Vec<DomainEvent> {
    let mut out = Vec::new();

    if matches!(ev.kind, EventKind::Other) {
        out.push(DomainEvent::file_with_reason(
            FileEventKind::ScopeRescan,
            ev.paths
                .first()
                .map(|p| p.to_string_lossy().replace('\\', "/"))
                .unwrap_or_default(),
            "platform-other",
        ));
        return out;
    }

    // Reliable rename: Modify(Name(Both)) with two paths.
    if let EventKind::Modify(ModifyKind::Name(rename_mode)) = ev.kind {
        match rename_mode {
            RenameMode::Both if ev.paths.len() >= 2 => {
                let from = path_string(&ev.paths[0]);
                let to = path_string(&ev.paths[1]);
                if rename_passes_filter(&ev.paths[0], &ev.paths[1], filter) {
                    out.push(DomainEvent::file_renamed(from, to));
                }
                return out;
            }
            RenameMode::Both | RenameMode::From | RenameMode::To | RenameMode::Any => {
                for path in &ev.paths {
                    maybe_push_path(&mut out, path, FileEventKind::FileModified, mode, filter);
                }
                out.push(DomainEvent::file_with_reason(
                    FileEventKind::ScopeRescan,
                    ev.paths.first().map(|p| path_string(p)).unwrap_or_default(),
                    "uncertain-rename",
                ));
                return out;
            }
            _ => {}
        }
    }

    if ev.paths.len() >= 2 {
        if let EventKind::Modify(ModifyKind::Name(_)) = ev.kind {
            let from = path_string(&ev.paths[0]);
            let to = path_string(&ev.paths[1]);
            if rename_passes_filter(&ev.paths[0], &ev.paths[1], filter) {
                out.push(DomainEvent::file_renamed(from, to));
            }
            return out;
        }
    }

    let kind = match ev.kind {
        EventKind::Create(_) => FileEventKind::FileCreated,
        EventKind::Remove(_) => FileEventKind::FileDeleted,
        EventKind::Modify(_) => FileEventKind::FileModified,
        EventKind::Access(_) | EventKind::Any | EventKind::Other => FileEventKind::FileModified,
    };

    for path in &ev.paths {
        maybe_push_path(&mut out, path, kind.clone(), mode, filter);
    }
    out
}

fn rename_passes_filter(from: &Path, to: &Path, filter: Option<&dyn Fn(&Path) -> bool>) -> bool {
    let Some(f) = filter else {
        return true;
    };
    from.is_dir() || to.is_dir() || f(from) || f(to)
}

fn maybe_push_path(
    out: &mut Vec<DomainEvent>,
    path: &Path,
    kind: FileEventKind,
    mode: DiscoveryMode,
    filter: Option<&dyn Fn(&Path) -> bool>,
) {
    let path_s = path_string(path);
    let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    if mode == DiscoveryMode::Workspace && TOPOLOGY_MANIFESTS.contains(&name) {
        out.push(DomainEvent::Workspace {
            kind: WorkspaceEventKind::TopologyChanged,
            node: None,
            path: Some(path_s),
        });
        return;
    }

    let is_dir = path.is_dir();
    if let Some(f) = filter {
        if !is_dir && !f(path) {
            return;
        }
    }

    out.push(DomainEvent::file(kind, path_s, None, None));
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// Collect recursive/non-recursive watch roots from scopes.
pub fn collect_watch_roots(scopes: &[argos_core::WatchScope]) -> Vec<(PathBuf, RecursiveMode)> {
    let mut roots = Vec::new();
    for scope in scopes {
        for rel in &scope.watch {
            let path = if rel == "." {
                PathBuf::from(&scope.root)
            } else {
                PathBuf::from(&scope.root).join(rel)
            };
            if path.is_dir() {
                roots.push((path, RecursiveMode::Recursive));
            } else if path.is_file() {
                if let Some(parent) = path.parent() {
                    roots.push((parent.to_path_buf(), RecursiveMode::NonRecursive));
                }
            }
        }
    }
    roots
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::event::CreateKind;

    #[test]
    fn maps_reliable_rename_both_paths() {
        let ev = Event {
            kind: EventKind::Modify(ModifyKind::Name(RenameMode::Both)),
            paths: vec![PathBuf::from("/lib/a.flac"), PathBuf::from("/lib/b.flac")],
            attrs: Default::default(),
        };
        let mapped = map_notify_event(ev, DiscoveryMode::Filesystem, None);
        assert_eq!(mapped.len(), 1);
        match &mapped[0] {
            DomainEvent::File {
                kind: FileEventKind::FileRenamed,
                from_path,
                to_path,
                ..
            } => {
                assert_eq!(from_path.as_deref(), Some("/lib/a.flac"));
                assert_eq!(to_path.as_deref(), Some("/lib/b.flac"));
            }
            other => panic!("expected rename, got {other:?}"),
        }
    }

    #[test]
    fn uncertain_rename_emits_rescan_reason() {
        let ev = Event {
            kind: EventKind::Modify(ModifyKind::Name(RenameMode::From)),
            paths: vec![PathBuf::from("/lib/a.flac")],
            attrs: Default::default(),
        };
        let mapped = map_notify_event(ev, DiscoveryMode::Filesystem, None);
        assert!(mapped.iter().any(|e| matches!(
            e,
            DomainEvent::File {
                kind: FileEventKind::ScopeRescan,
                reason: Some(r),
                ..
            } if r == "uncertain-rename"
        )));
    }

    #[test]
    fn filesystem_mode_skips_topology_manifest_special_case() {
        let ev = Event {
            kind: EventKind::Create(CreateKind::File),
            paths: vec![PathBuf::from("/lib/package.json")],
            attrs: Default::default(),
        };
        let mapped = map_notify_event(ev, DiscoveryMode::Filesystem, None);
        assert!(mapped
            .iter()
            .all(|e| !matches!(e, DomainEvent::Workspace { .. })));
    }

    #[test]
    fn overflow_flag_drains_as_scope_rescan() {
        let dir = tempfile::tempdir().unwrap();
        let (_watcher, queue) =
            start_bounded_watcher(8, &[(dir.path().to_path_buf(), RecursiveMode::Recursive)])
                .unwrap();
        queue.force_overflow();
        let events = queue
            .drain(DiscoveryMode::Filesystem, dir.path(), None)
            .unwrap();
        assert!(events.iter().any(|e| matches!(
            e,
            DomainEvent::File {
                kind: FileEventKind::ScopeRescan,
                reason: Some(r),
                ..
            } if r == "overflow"
        )));
    }
}
