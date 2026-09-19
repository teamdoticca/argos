//! Node-API bindings for Argos. Snapshot remains sole business truth.

use argos_backend::WatchBackend;
use argos_core::{
    compute_delta as compute_delta_core, explain_path, find_owner, list_nodes, list_scopes,
    workspace_health, DomainEvent, OpenOptions, Workspace as CoreWorkspace,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use parking_lot::Mutex;
use std::path::PathBuf;

struct WatchSession {
    backend: Box<dyn WatchBackend>,
}

#[napi]
pub struct Workspace {
    inner: CoreWorkspace,
    watch: Mutex<Option<WatchSession>>,
}

#[napi]
impl Workspace {
    #[napi(factory)]
    pub fn open(root: String, options_json: Option<String>) -> Result<Self> {
        let options = parse_open_options(options_json.as_deref())?;
        let inner = CoreWorkspace::open(&root, options).map_err(to_napi_err)?;
        Ok(Self {
            inner,
            watch: Mutex::new(None),
        })
    }

    #[napi(getter)]
    pub fn current_snapshot(&self) -> Result<serde_json::Value> {
        let snap = self.inner.current_snapshot();
        serde_json::to_value(snap.as_ref()).map_err(to_napi_err)
    }

    #[napi]
    pub fn rebuild_snapshot(&self) -> Result<serde_json::Value> {
        let snap = self.inner.rebuild_snapshot().map_err(to_napi_err)?;
        serde_json::to_value(snap.as_ref()).map_err(to_napi_err)
    }

    #[napi]
    pub fn list_nodes(&self) -> Result<serde_json::Value> {
        let snap = self.inner.current_snapshot();
        let nodes: Vec<_> = list_nodes(&snap).into_iter().cloned().collect();
        serde_json::to_value(nodes).map_err(to_napi_err)
    }

    #[napi]
    pub fn list_scopes(&self) -> Result<serde_json::Value> {
        let snap = self.inner.current_snapshot();
        let scopes = list_scopes(&snap);
        serde_json::to_value(scopes).map_err(to_napi_err)
    }

    #[napi]
    pub fn find_owner(&self, path: String) -> Result<serde_json::Value> {
        let snap = self.inner.current_snapshot();
        let hit = find_owner(&snap, PathBuf::from(path).as_path());
        serde_json::to_value(hit).map_err(to_napi_err)
    }

    #[napi]
    pub fn explain_path(&self, path: String) -> Result<serde_json::Value> {
        let snap = self.inner.current_snapshot();
        let p = PathBuf::from(&path);
        let ignored = self.inner.ignore().is_ignored(&p);
        let explained = explain_path(&snap, &p, ignored, None);
        serde_json::to_value(explained).map_err(to_napi_err)
    }

    #[napi]
    pub fn health(&self) -> Result<serde_json::Value> {
        let snap = self.inner.current_snapshot();
        serde_json::to_value(workspace_health(&snap)).map_err(to_napi_err)
    }

    #[napi]
    pub fn watch_start(&self) -> Result<()> {
        let scopes = self.inner.current_snapshot().planning.scopes.clone();
        let mut backend = create_native_backend();
        backend.start(&self.inner, &scopes).map_err(to_napi_err)?;
        *self.watch.lock() = Some(WatchSession { backend });
        Ok(())
    }

    #[napi]
    pub fn watch_stop(&self) -> Result<()> {
        if let Some(mut session) = self.watch.lock().take() {
            session.backend.stop().map_err(to_napi_err)?;
        }
        Ok(())
    }

    #[napi]
    pub fn watch_poll(&self) -> Result<serde_json::Value> {
        let mut guard = self.watch.lock();
        let Some(session) = guard.as_mut() else {
            return Err(Error::from_reason("watch not started"));
        };
        let events = session.backend.poll().map_err(to_napi_err)?;
        let needs_recovery = events.iter().any(|e| {
            matches!(
                e,
                DomainEvent::File {
                    kind: argos_core::FileEventKind::ScopeRescan,
                    ..
                }
            )
        });
        let topology_changed = events.iter().any(|e| {
            matches!(
                e,
                DomainEvent::Workspace {
                    kind: argos_core::WorkspaceEventKind::TopologyChanged,
                    ..
                }
            )
        });
        drop(guard);

        if needs_recovery {
            let _ = self.inner.mark_recovering();
            let _ = self.inner.mark_ready_from_rebuild();
        } else if topology_changed {
            let _ = self.inner.rebuild_snapshot();
        }

        let snap = self.inner.current_snapshot();
        let enriched: Vec<DomainEvent> = events
            .into_iter()
            .map(|ev| enrich_event(&snap, ev))
            .collect();
        serde_json::to_value(enriched).map_err(to_napi_err)
    }

    #[napi]
    pub fn get_affected_scopes(&self, event: serde_json::Value) -> Result<Vec<String>> {
        let event: DomainEvent = serde_json::from_value(event).map_err(to_napi_err)?;
        let snap = self.inner.current_snapshot();
        let mut affected = Vec::new();
        match &event {
            DomainEvent::File {
                path, scope, node, ..
            } => {
                if let Some(s) = scope {
                    affected.push(s.clone());
                } else if let Some(n) = node {
                    affected.push(n.clone());
                } else if let Some(o) = find_owner(&snap, PathBuf::from(path).as_path()) {
                    affected.push(o.node_id);
                }
            }
            DomainEvent::Workspace { node, .. } => {
                if let Some(n) = node {
                    affected.push(n.clone());
                } else {
                    affected.extend(snap.planning.scopes.iter().map(|s| s.package.clone()));
                }
            }
        }
        Ok(affected)
    }

    #[napi]
    pub fn close(&self) -> Result<()> {
        self.watch_stop()
    }
}

#[napi]
pub fn compute_delta(
    old_snapshot: serde_json::Value,
    new_snapshot: serde_json::Value,
) -> Result<serde_json::Value> {
    let old: argos_core::WorkspaceSnapshot =
        serde_json::from_value(old_snapshot).map_err(to_napi_err)?;
    let new: argos_core::WorkspaceSnapshot =
        serde_json::from_value(new_snapshot).map_err(to_napi_err)?;
    let delta = compute_delta_core(&old, &new).map_err(to_napi_err)?;
    serde_json::to_value(delta).map_err(to_napi_err)
}

fn parse_open_options(options_json: Option<&str>) -> Result<OpenOptions> {
    let Some(raw) = options_json.filter(|s| !s.trim().is_empty() && *s != "{}") else {
        return Ok(OpenOptions::default());
    };
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Opts {
        symlink_mode: Option<String>,
    }
    let opts: Opts = serde_json::from_str(raw).map_err(to_napi_err)?;
    Ok(OpenOptions {
        symlink_mode: opts.symlink_mode,
    })
}

fn to_napi_err(e: impl ToString) -> Error {
    Error::from_reason(e.to_string())
}

fn create_native_backend() -> Box<dyn WatchBackend> {
    #[cfg(windows)]
    {
        Box::new(argos_backend_windows::WindowsBackend::new())
    }
    #[cfg(target_os = "linux")]
    {
        Box::new(argos_backend_linux::LinuxBackend::new())
    }
    #[cfg(target_os = "macos")]
    {
        Box::new(argos_backend_macos::MacosBackend::new())
    }
    #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
    {
        Box::new(UnsupportedBackend)
    }
}

#[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
struct UnsupportedBackend;

#[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
impl WatchBackend for UnsupportedBackend {
    fn name(&self) -> &str {
        "unsupported"
    }
    fn capabilities(&self) -> argos_backend::PlatformCapabilities {
        argos_backend::PlatformCapabilities {
            recursive_watch: false,
            rename_tracking: false,
            overflow_notifications: false,
            native_journaling: false,
            symlink_handling: false,
            max_watch_resources: None,
        }
    }
    fn start(&mut self, _: &CoreWorkspace, _: &[argos_core::WatchScope]) -> argos_core::Result<()> {
        Err(argos_core::ArgosError::UnsupportedPlatform(
            "no watch backend for this OS".into(),
        ))
    }
    fn stop(&mut self) -> argos_core::Result<()> {
        Ok(())
    }
    fn poll(&mut self) -> argos_core::Result<Vec<DomainEvent>> {
        Ok(vec![])
    }
    fn watched_roots(&self) -> Vec<PathBuf> {
        vec![]
    }
}

fn enrich_event(snapshot: &argos_core::WorkspaceSnapshot, event: DomainEvent) -> DomainEvent {
    match event {
        DomainEvent::File {
            kind,
            path,
            node,
            scope,
        } => {
            let owner = find_owner(snapshot, PathBuf::from(&path).as_path());
            let node = node.or_else(|| owner.as_ref().map(|o| o.node_id.clone()));
            let scope = scope.or_else(|| {
                owner.as_ref().and_then(|o| {
                    snapshot
                        .planning
                        .scopes
                        .iter()
                        .find(|s| s.package == o.node_id)
                        .map(|s| s.root.clone())
                })
            });
            DomainEvent::File {
                kind,
                path,
                node,
                scope,
            }
        }
        other => other,
    }
}
