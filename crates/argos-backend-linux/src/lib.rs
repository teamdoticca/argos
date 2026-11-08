//! Linux backend using inotify (via the `notify` crate).

use argos_backend::{
    collect_watch_roots, start_bounded_watcher, BoundedWatchQueue, PlatformCapabilities,
    WatchBackend,
};
use argos_core::{DomainEvent, FileEventKind, Result, WatchScope, Workspace};
use notify::RecommendedWatcher;
use std::path::PathBuf;

pub struct LinuxBackend {
    watcher: Option<RecommendedWatcher>,
    queue: Option<BoundedWatchQueue>,
    roots: Vec<PathBuf>,
    mode: argos_core::DiscoveryMode,
    root: PathBuf,
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
            queue: None,
            roots: vec![],
            mode: argos_core::DiscoveryMode::Workspace,
            root: PathBuf::new(),
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
            let roots = collect_watch_roots(scopes);
            let capacity = workspace.event_buffer_capacity();
            let (watcher, queue) = start_bounded_watcher(capacity, &roots)?;
            workspace.set_backend_name(Some(self.name().into()));
            self.roots = roots.into_iter().map(|(p, _)| p).collect();
            self.mode = workspace.mode();
            self.root = workspace.root().to_path_buf();
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
        #[cfg(not(target_os = "linux"))]
        {
            Err(argos_core::ArgosError::UnsupportedPlatform(
                "linux backend requires Linux host".into(),
            ))
        }
        #[cfg(target_os = "linux")]
        {
            let Some(queue) = &self.queue else {
                return Err(argos_core::ArgosError::WatchNotStarted);
            };
            queue.drain(self.mode, &self.root, None)
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
