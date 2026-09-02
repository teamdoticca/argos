//! Argos watch backend abstraction and PlatformCapabilities.

use argos_core::{DomainEvent, Result, WatchScope, Workspace};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformCapabilities {
    pub recursive_watch: bool,
    pub rename_tracking: bool,
    pub overflow_notifications: bool,
    pub native_journaling: bool,
    pub symlink_handling: bool,
    pub max_watch_resources: Option<u32>,
}

pub trait WatchBackend: Send {
    fn name(&self) -> &str;
    fn capabilities(&self) -> PlatformCapabilities;
    fn start(&mut self, workspace: &Workspace, scopes: &[WatchScope]) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn poll(&mut self) -> Result<Vec<DomainEvent>>;
    fn watched_roots(&self) -> Vec<PathBuf>;
}

pub fn select_backend_name() -> &'static str {
    if cfg!(windows) {
        "rdcw"
    } else if cfg!(target_os = "macos") {
        "fsevents"
    } else {
        "inotify"
    }
}

/// Platform recovery: map overflow / invalidation to affected-scope rescan.
pub mod recovery {
    use argos_core::{DomainEvent, FileEventKind, WatchScope, Workspace};

    pub fn is_overflow(event: &DomainEvent) -> bool {
        matches!(
            event,
            DomainEvent::File {
                kind: FileEventKind::ScopeRescan,
                ..
            }
        )
    }

    /// Recover by marking recovering, rebuilding snapshot (scoped planner), returning synthetic events.
    pub fn recover_from_overflow(
        workspace: &Workspace,
        affected_scopes: &[WatchScope],
    ) -> argos_core::Result<Vec<DomainEvent>> {
        let _ = workspace.mark_recovering()?;
        let _ = workspace.mark_ready_from_rebuild()?;
        let events = affected_scopes
            .iter()
            .map(|s| DomainEvent::File {
                kind: FileEventKind::ScopeRescan,
                path: s.root.clone(),
                node: Some(s.package.clone()),
                scope: Some(s.root.clone()),
            })
            .collect();
        Ok(events)
    }
}
