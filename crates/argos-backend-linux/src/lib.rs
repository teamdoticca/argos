//! Linux backend using inotify (via the `notify` crate).

use argos_backend::{PlatformCapabilities, WatchBackend};
#[cfg(target_os = "linux")]
use argos_core::WorkspaceEventKind;
use argos_core::{DomainEvent, FileEventKind, Result, WatchScope, Workspace};
use notify::RecommendedWatcher;
#[cfg(target_os = "linux")]
use notify::{Config, EventKind, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
#[cfg(target_os = "linux")]
use std::sync::mpsc::TryRecvError;

pub struct LinuxBackend {
    watcher: Option<RecommendedWatcher>,
    rx: Option<Receiver<notify::Result<notify::Event>>>,
    roots: Vec<PathBuf>,
}

impl Default for LinuxBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl LinuxBackend {
    pub fn new() -> Self {
        Self {
            watcher: None,
            rx: None,
            roots: vec![],
        }
    }
}

impl WatchBackend for LinuxBackend {
    fn name(&self) -> &str {
        "inotify"
    }

    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities {
            recursive_watch: true,
            rename_tracking: true,
            overflow_notifications: true,
            native_journaling: false,
            symlink_handling: true,
            max_watch_resources: Some(8192),
        }
    }

    fn start(&mut self, workspace: &Workspace, scopes: &[WatchScope]) -> Result<()> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (workspace, scopes);
            Err(argos_core::ArgosError::UnsupportedPlatform(
                "linux backend requires Linux host".into(),
            ))
        }
        #[cfg(target_os = "linux")]
        {
            start_inner(self, workspace, scopes)
        }
    }

    fn stop(&mut self) -> Result<()> {
        self.watcher = None;
        self.rx = None;
        self.roots.clear();
        Ok(())
    }

    fn poll(&mut self) -> Result<Vec<DomainEvent>> {
        #[cfg(not(target_os = "linux"))]
        {
            Err(argos_core::ArgosError::UnsupportedPlatform(
                "linux backend requires Linux host".into(),
            ))
        }
        #[cfg(target_os = "linux")]
        {
            poll_inner(self)
        }
    }

    fn watched_roots(&self) -> Vec<PathBuf> {
        self.roots.clone()
    }
}

#[cfg(target_os = "linux")]
fn start_inner(
    this: &mut LinuxBackend,
    workspace: &Workspace,
    scopes: &[WatchScope],
) -> Result<()> {
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
                        .map_err(|e| argos_core::ArgosError::Message(format!("watch: {e}")))?;
                    roots.push(parent.to_path_buf());
                }
            }
        }
    }
    workspace.set_backend_name(Some(this.name().into()));
    this.watcher = Some(watcher);
    this.rx = Some(rx);
    this.roots = roots;
    Ok(())
}

#[cfg(target_os = "linux")]
fn poll_inner(this: &mut LinuxBackend) -> Result<Vec<DomainEvent>> {
    let Some(rx) = &this.rx else {
        return Err(argos_core::ArgosError::WatchNotStarted);
    };
    let mut events = Vec::new();
    loop {
        match rx.try_recv() {
            Ok(Ok(ev)) => {
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
                        _ => FileEventKind::FileModified,
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
                if msg.contains("overflow") || msg.contains("q_overflow") {
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

pub fn is_overflow_event(event: &DomainEvent) -> bool {
    matches!(
        event,
        DomainEvent::File {
            kind: FileEventKind::ScopeRescan,
            ..
        }
    )
}
