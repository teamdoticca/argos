//! Windows backend using ReadDirectoryChangesW (via the `notify` crate).

use argos_backend::{PlatformCapabilities, WatchBackend};
use argos_core::{
    DomainEvent, FileEventKind, Result, WatchScope, Workspace, WorkspaceEventKind,
};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, TryRecvError};

pub struct WindowsBackend {
    watcher: Option<RecommendedWatcher>,
    rx: Option<Receiver<notify::Result<notify::Event>>>,
    roots: Vec<PathBuf>,
}

impl Default for WindowsBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowsBackend {
    pub fn new() -> Self {
        Self {
            watcher: None,
            rx: None,
            roots: vec![],
        }
    }
}

impl WatchBackend for WindowsBackend {
    fn name(&self) -> &str {
        "rdcw"
    }

    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities {
            recursive_watch: true,
            rename_tracking: true,
            overflow_notifications: true,
            native_journaling: false,
            symlink_handling: true,
            max_watch_resources: None,
        }
    }

    fn start(&mut self, workspace: &Workspace, scopes: &[WatchScope]) -> Result<()> {
        #[cfg(not(windows))]
        {
            let _ = (workspace, scopes);
            return Err(argos_core::ArgosError::UnsupportedPlatform(
                "windows backend requires Windows host".into(),
            ));
        }
        #[cfg(windows)]
        {
            let (tx, rx) = std::sync::mpsc::channel();
            let mut watcher = RecommendedWatcher::new(tx, Config::default())
                .map_err(|e| argos_core::ArgosError::Message(format!("watcher: {e}")))?;

            let mut roots = Vec::new();
            for scope in scopes {
                for rel in &scope.watch {
                    let path = if rel == "." {
                        PathBuf::from(&scope.root)
                    } else {
                        PathBuf::from(&scope.root).join(rel)
                    };
                    if path.is_dir() {
                        watcher
                            .watch(&path, RecursiveMode::Recursive)
                            .map_err(|e| argos_core::ArgosError::Message(format!("watch: {e}")))?;
                        roots.push(path);
                    } else if path.is_file() {
                        if let Some(parent) = path.parent() {
                            watcher
                                .watch(parent, RecursiveMode::NonRecursive)
                                .map_err(|e| {
                                    argos_core::ArgosError::Message(format!("watch: {e}"))
                                })?;
                            roots.push(parent.to_path_buf());
                        }
                    }
                }
            }

            for name in [
                "pnpm-workspace.yaml",
                "package.json",
                "Cargo.toml",
                ".workspace",
                ".gitmodules",
            ] {
                let p = workspace.root().join(name);
                if p.is_file() {
                    if let Some(parent) = p.parent() {
                        let _ = watcher.watch(parent, RecursiveMode::NonRecursive);
                    }
                }
            }

            workspace.set_backend_name(Some(self.name().into()));
            self.watcher = Some(watcher);
            self.rx = Some(rx);
            self.roots = roots;
            Ok(())
        }
    }

    fn stop(&mut self) -> Result<()> {
        self.watcher = None;
        self.rx = None;
        self.roots.clear();
        Ok(())
    }

    fn poll(&mut self) -> Result<Vec<DomainEvent>> {
        #[cfg(not(windows))]
        {
            return Err(argos_core::ArgosError::UnsupportedPlatform(
                "windows backend requires Windows host".into(),
            ));
        }
        #[cfg(windows)]
        {
            let Some(rx) = &self.rx else {
                return Err(argos_core::ArgosError::WatchNotStarted);
            };
            let mut events = Vec::new();
            loop {
                match rx.try_recv() {
                    Ok(Ok(ev)) => {
                        if matches!(ev.kind, EventKind::Other) {
                            events.push(DomainEvent::File {
                                kind: FileEventKind::ScopeRescan,
                                path: ev
                                    .paths
                                    .first()
                                    .map(|p| p.to_string_lossy().replace('\\', "/"))
                                    .unwrap_or_default(),
                                node: None,
                                scope: None,
                            });
                            continue;
                        }
                        for path in ev.paths {
                            let path_s = path.to_string_lossy().replace('\\', "/");
                            let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                            if matches!(
                                name,
                                "pnpm-workspace.yaml"
                                    | "package.json"
                                    | "Cargo.toml"
                                    | ".workspace"
                                    | ".gitmodules"
                            ) {
                                events.push(DomainEvent::Workspace {
                                    kind: WorkspaceEventKind::TopologyChanged,
                                    node: None,
                                    path: Some(path_s),
                                });
                                continue;
                            }
                            let kind = match ev.kind {
                                EventKind::Create(_) => FileEventKind::FileCreated,
                                EventKind::Modify(_) => FileEventKind::FileModified,
                                EventKind::Remove(_) => FileEventKind::FileDeleted,
                                EventKind::Any | EventKind::Access(_) | EventKind::Other => {
                                    FileEventKind::FileModified
                                }
                            };
                            events.push(DomainEvent::File {
                                kind,
                                path: path_s,
                                node: None,
                                scope: None,
                            });
                        }
                    }
                    Ok(Err(e)) => {
                        let msg = e.to_string().to_lowercase();
                        if msg.contains("overflow")
                            || msg.contains("buffer")
                            || msg.contains("notify_enum")
                        {
                            events.push(DomainEvent::File {
                                kind: FileEventKind::ScopeRescan,
                                path: String::new(),
                                node: None,
                                scope: None,
                            });
                        } else {
                            return Err(argos_core::ArgosError::Message(format!("notify: {e}")));
                        }
                    }
                    Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => break,
                }
            }
            Ok(events)
        }
    }

    fn watched_roots(&self) -> Vec<PathBuf> {
        self.roots.clone()
    }
}

pub fn is_overflow_event(event: &DomainEvent) -> bool {
    matches!(
        event,
        DomainEvent::File {
            kind: FileEventKind::ScopeRescan,
            ..
        }
    )
}
