//! Windows backend using ReadDirectoryChangesW (via the `notify` crate).

#[cfg(windows)]
use argos_backend::{collect_watch_roots, start_bounded_watcher};
use argos_backend::{BoundedWatchQueue, PlatformCapabilities, WatchBackend};
use argos_core::{DomainEvent, FileEventKind, Result, WatchScope, Workspace};
use notify::RecommendedWatcher;
use std::path::PathBuf;

#[cfg_attr(not(windows), allow(dead_code))]
pub struct WindowsBackend {
    watcher: Option<RecommendedWatcher>,
    queue: Option<BoundedWatchQueue>,
    roots: Vec<PathBuf>,
    mode: argos_core::DiscoveryMode,
    root: PathBuf,
    capacity: usize,
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
            queue: None,
            roots: vec![],
            mode: argos_core::DiscoveryMode::Workspace,
            root: PathBuf::new(),
            capacity: argos_core::DEFAULT_FILESYSTEM_EVENT_BUFFER,
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
            Err(argos_core::ArgosError::UnsupportedPlatform(
                "windows backend requires Windows host".into(),
            ))
        }
        #[cfg(windows)]
        {
            let mut roots = collect_watch_roots(scopes);
            if workspace.mode() == argos_core::DiscoveryMode::Workspace {
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
                            roots.push((parent.to_path_buf(), notify::RecursiveMode::NonRecursive));
                        }
                    }
                }
            }
            let capacity = workspace.event_buffer_capacity();
            let (watcher, queue) = start_bounded_watcher(capacity, &roots)?;
            workspace.set_backend_name(Some(self.name().into()));
            self.roots = roots.into_iter().map(|(p, _)| p).collect();
            self.mode = workspace.mode();
            self.root = workspace.root().to_path_buf();
            self.capacity = capacity;
            self.watcher = Some(watcher);
            self.queue = Some(queue);
            Ok(())
        }
    }

    fn stop(&mut self) -> Result<()> {
        self.watcher = None;
        self.queue = None;
        self.roots.clear();
        Ok(())
    }

    fn poll(&mut self) -> Result<Vec<DomainEvent>> {
        #[cfg(not(windows))]
        {
            Err(argos_core::ArgosError::UnsupportedPlatform(
                "windows backend requires Windows host".into(),
            ))
        }
        #[cfg(windows)]
        {
            let Some(queue) = &self.queue else {
                return Err(argos_core::ArgosError::WatchNotStarted);
            };
            let root = self.root.clone();
            let mode = self.mode;
            queue.drain(mode, &root, None)
        }
    }

    fn watched_roots(&self) -> Vec<PathBuf> {
        self.roots.clone()
    }

    fn inject_overflow_for_test(&mut self) {
        if let Some(q) = &self.queue {
            q.force_overflow();
        }
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
