//! Argos C ABI.
//!
//! Handles are opaque pointers owned by the consumer until free is called.

use argos_backend::WatchBackend;
use argos_core::{
    compute_delta, explain_path, explain_scope, find_owner, list_nodes, list_scopes,
    snapshot_validate, workspace_health, DomainEvent, OpenOptions, Workspace, WorkspaceSnapshot,
};
use parking_lot::Mutex;
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;

struct WatchSession {
    backend: Box<dyn WatchBackend>,
}

struct WorkspaceHandle {
    workspace: Workspace,
    watch: Mutex<Option<WatchSession>>,
}

thread_local! {
    static LAST_ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
}

fn set_last_error(msg: &str) {
    LAST_ERROR.with(|e| *e.borrow_mut() = Some(msg.to_string()));
}

unsafe fn cstr_to_path(p: *const c_char) -> Result<PathBuf, String> {
    if p.is_null() {
        return Err("null path".into());
    }
    let s = CStr::from_ptr(p).to_string_lossy().into_owned();
    Ok(PathBuf::from(s))
}

unsafe fn write_string(out: *mut *mut c_char, value: &str) -> c_int {
    if out.is_null() {
        return -1;
    }
    match CString::new(value) {
        Ok(c) => {
            *out = c.into_raw();
            0
        }
        Err(_) => {
            set_last_error("string contains interior nul");
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn argos_string_free(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

#[no_mangle]
pub unsafe extern "C" fn argos_last_error(out: *mut *mut c_char) -> c_int {
    let msg = LAST_ERROR.with(|e| e.borrow().clone().unwrap_or_default());
    write_string(out, &msg)
}

#[no_mangle]
pub unsafe extern "C" fn argos_workspace_open(
    root: *const c_char,
    options_json: *const c_char,
    out_workspace: *mut *mut c_void,
) -> c_int {
    if out_workspace.is_null() {
        set_last_error("null out_workspace");
        return -1;
    }
    let path = match cstr_to_path(root) {
        Ok(p) => p,
        Err(e) => {
            set_last_error(&e);
            return -1;
        }
    };
    let mut options = OpenOptions::default();
    if !options_json.is_null() {
        let raw = CStr::from_ptr(options_json).to_string_lossy();
        if !raw.is_empty() {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw) {
                if let Some(m) = v.get("symlinkMode").and_then(|x| x.as_str()) {
                    options.symlink_mode = Some(m.to_string());
                }
            }
        }
    }
    match Workspace::open(&path, options) {
        Ok(ws) => {
            let handle = Box::new(WorkspaceHandle {
                workspace: ws,
                watch: Mutex::new(None),
            });
            *out_workspace = Box::into_raw(handle) as *mut c_void;
            0
        }
        Err(e) => {
            set_last_error(&e.to_string());
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn argos_workspace_free(workspace: *mut c_void) {
    if !workspace.is_null() {
        drop(Box::from_raw(workspace as *mut WorkspaceHandle));
    }
}

unsafe fn as_handle<'a>(workspace: *mut c_void) -> Result<&'a mut WorkspaceHandle, String> {
    if workspace.is_null() {
        return Err("null workspace".into());
    }
    Ok(&mut *(workspace as *mut WorkspaceHandle))
}

#[no_mangle]
pub unsafe extern "C" fn argos_workspace_current_snapshot(
    workspace: *mut c_void,
    out_json: *mut *mut c_char,
) -> c_int {
    let handle = match as_handle(workspace) {
        Ok(h) => h,
        Err(e) => {
            set_last_error(&e);
            return -1;
        }
    };
    match handle.workspace.current_snapshot().to_json() {
        Ok(j) => write_string(out_json, &j),
        Err(e) => {
            set_last_error(&e.to_string());
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn argos_workspace_rebuild_snapshot(
    workspace: *mut c_void,
    out_json: *mut *mut c_char,
) -> c_int {
    let handle = match as_handle(workspace) {
        Ok(h) => h,
        Err(e) => {
            set_last_error(&e);
            return -1;
        }
    };
    match handle.workspace.rebuild_snapshot() {
        Ok(snap) => match snap.to_json() {
            Ok(j) => write_string(out_json, &j),
            Err(e) => {
                set_last_error(&e.to_string());
                -1
            }
        },
        Err(e) => {
            set_last_error(&e.to_string());
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn argos_snapshot_validate(
    snapshot_json: *const c_char,
    out_report: *mut *mut c_char,
) -> c_int {
    if snapshot_json.is_null() {
        set_last_error("null snapshot");
        return -1;
    }
    let raw = CStr::from_ptr(snapshot_json).to_string_lossy();
    let snap: WorkspaceSnapshot = match serde_json::from_str(&raw) {
        Ok(s) => s,
        Err(e) => {
            set_last_error(&e.to_string());
            return -1;
        }
    };
    let report = snapshot_validate(&snap);
    match serde_json::to_string(&report) {
        Ok(j) => write_string(out_report, &j),
        Err(e) => {
            set_last_error(&e.to_string());
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn argos_explain_path(
    workspace: *mut c_void,
    path: *const c_char,
    out_json: *mut *mut c_char,
) -> c_int {
    let handle = match as_handle(workspace) {
        Ok(h) => h,
        Err(e) => {
            set_last_error(&e);
            return -1;
        }
    };
    let p = match cstr_to_path(path) {
        Ok(p) => p,
        Err(e) => {
            set_last_error(&e);
            return -1;
        }
    };
    let snap = handle.workspace.current_snapshot();
    let ignored = handle.workspace.ignore().is_ignored(&p);
    let rule = handle
        .workspace
        .ignore()
        .explain(&p)
        .map(|r| (r.pattern.clone(), r.source.clone()));
    let result = explain_path(&snap, &p, ignored, rule);
    match serde_json::to_string(&result) {
        Ok(j) => write_string(out_json, &j),
        Err(e) => {
            set_last_error(&e.to_string());
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn argos_explain_scope(
    workspace: *mut c_void,
    scope: *const c_char,
    out_json: *mut *mut c_char,
) -> c_int {
    let handle = match as_handle(workspace) {
        Ok(h) => h,
        Err(e) => {
            set_last_error(&e);
            return -1;
        }
    };
    let scope_s = if scope.is_null() {
        String::new()
    } else {
        CStr::from_ptr(scope).to_string_lossy().into_owned()
    };
    let snap = handle.workspace.current_snapshot();
    let result = explain_scope(&snap, &scope_s);
    match serde_json::to_string(&result) {
        Ok(j) => write_string(out_json, &j),
        Err(e) => {
            set_last_error(&e.to_string());
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn argos_find_owner(
    workspace: *mut c_void,
    path: *const c_char,
    out_json: *mut *mut c_char,
) -> c_int {
    let handle = match as_handle(workspace) {
        Ok(h) => h,
        Err(e) => {
            set_last_error(&e);
            return -1;
        }
    };
    let p = match cstr_to_path(path) {
        Ok(p) => p,
        Err(e) => {
            set_last_error(&e);
            return -1;
        }
    };
    let snap = handle.workspace.current_snapshot();
    let hit = find_owner(&snap, &p);
    match serde_json::to_string(&hit) {
        Ok(j) => write_string(out_json, &j),
        Err(e) => {
            set_last_error(&e.to_string());
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn argos_list_nodes(
    workspace: *mut c_void,
    out_json: *mut *mut c_char,
) -> c_int {
    let handle = match as_handle(workspace) {
        Ok(h) => h,
        Err(e) => {
            set_last_error(&e);
            return -1;
        }
    };
    let snap = handle.workspace.current_snapshot();
    let nodes: Vec<_> = list_nodes(&snap).into_iter().cloned().collect();
    match serde_json::to_string(&nodes) {
        Ok(j) => write_string(out_json, &j),
        Err(e) => {
            set_last_error(&e.to_string());
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn argos_list_scopes(
    workspace: *mut c_void,
    out_json: *mut *mut c_char,
) -> c_int {
    let handle = match as_handle(workspace) {
        Ok(h) => h,
        Err(e) => {
            set_last_error(&e);
            return -1;
        }
    };
    let snap = handle.workspace.current_snapshot();
    match serde_json::to_string(list_scopes(&snap)) {
        Ok(j) => write_string(out_json, &j),
        Err(e) => {
            set_last_error(&e.to_string());
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn argos_workspace_health(
    workspace: *mut c_void,
    out_json: *mut *mut c_char,
) -> c_int {
    let handle = match as_handle(workspace) {
        Ok(h) => h,
        Err(e) => {
            set_last_error(&e);
            return -1;
        }
    };
    let snap = handle.workspace.current_snapshot();
    match serde_json::to_string(&workspace_health(&snap)) {
        Ok(j) => write_string(out_json, &j),
        Err(e) => {
            set_last_error(&e.to_string());
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn argos_platform_info(
    workspace: *mut c_void,
    out_json: *mut *mut c_char,
) -> c_int {
    let handle = match as_handle(workspace) {
        Ok(h) => h,
        Err(e) => {
            set_last_error(&e);
            return -1;
        }
    };
    match serde_json::to_string(&handle.workspace.platform_info()) {
        Ok(j) => write_string(out_json, &j),
        Err(e) => {
            set_last_error(&e.to_string());
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn argos_compute_delta(
    old_json: *const c_char,
    new_json: *const c_char,
    out_json: *mut *mut c_char,
) -> c_int {
    if old_json.is_null() || new_json.is_null() {
        set_last_error("null snapshot json");
        return -1;
    }
    let old_s = CStr::from_ptr(old_json).to_string_lossy();
    let new_s = CStr::from_ptr(new_json).to_string_lossy();
    let old: WorkspaceSnapshot = match serde_json::from_str(&old_s) {
        Ok(s) => s,
        Err(e) => {
            set_last_error(&e.to_string());
            return -1;
        }
    };
    let new: WorkspaceSnapshot = match serde_json::from_str(&new_s) {
        Ok(s) => s,
        Err(e) => {
            set_last_error(&e.to_string());
            return -1;
        }
    };
    match compute_delta(&old, &new) {
        Ok(d) => match serde_json::to_string(&d) {
            Ok(j) => write_string(out_json, &j),
            Err(e) => {
                set_last_error(&e.to_string());
                -1
            }
        },
        Err(e) => {
            set_last_error(&e.to_string());
            -1
        }
    }
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
        Box::new(NullBackend)
    }
}

#[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
struct NullBackend;

#[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
impl WatchBackend for NullBackend {
    fn name(&self) -> &str {
        "null"
    }
    fn capabilities(&self) -> argos_backend::PlatformCapabilities {
        argos_backend::PlatformCapabilities {
            recursive_watch: false,
            rename_tracking: false,
            overflow_notifications: false,
            native_journaling: false,
            symlink_handling: false,
            max_watch_resources: Some(0),
        }
    }
    fn start(
        &mut self,
        _: &Workspace,
        _: &[argos_core::WatchScope],
    ) -> argos_core::Result<()> {
        Err(argos_core::ArgosError::UnsupportedPlatform(
            "no backend".into(),
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

#[no_mangle]
pub unsafe extern "C" fn argos_watch_start(workspace: *mut c_void) -> c_int {
    let handle = match as_handle(workspace) {
        Ok(h) => h,
        Err(e) => {
            set_last_error(&e);
            return -1;
        }
    };
    let scopes = handle.workspace.current_snapshot().planning.scopes.clone();
    let mut backend = create_native_backend();
    if let Err(e) = backend.start(&handle.workspace, &scopes) {
        set_last_error(&e.to_string());
        return -1;
    }
    *handle.watch.lock() = Some(WatchSession { backend });
    0
}

#[no_mangle]
pub unsafe extern "C" fn argos_watch_stop(workspace: *mut c_void) -> c_int {
    let handle = match as_handle(workspace) {
        Ok(h) => h,
        Err(e) => {
            set_last_error(&e);
            return -1;
        }
    };
    if let Some(mut session) = handle.watch.lock().take() {
        if let Err(e) = session.backend.stop() {
            set_last_error(&e.to_string());
            return -1;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn argos_watch_poll(
    workspace: *mut c_void,
    out_json: *mut *mut c_char,
) -> c_int {
    let handle = match as_handle(workspace) {
        Ok(h) => h,
        Err(e) => {
            set_last_error(&e);
            return -1;
        }
    };
    let mut guard = handle.watch.lock();
    let Some(session) = guard.as_mut() else {
        set_last_error("watch not started");
        return -1;
    };
    let events = match session.backend.poll() {
        Ok(e) => e,
        Err(e) => {
            set_last_error(&e.to_string());
            return -1;
        }
    };

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
        let _ = handle.workspace.mark_recovering();
        let _ = handle.workspace.mark_ready_from_rebuild();
    } else if topology_changed {
        let _ = handle.workspace.rebuild_snapshot();
    }

    let snap = handle.workspace.current_snapshot();
    let enriched: Vec<DomainEvent> = events
        .into_iter()
        .map(|ev| enrich_event(&snap, ev))
        .collect();

    match serde_json::to_string(&enriched) {
        Ok(j) => write_string(out_json, &j),
        Err(e) => {
            set_last_error(&e.to_string());
            -1
        }
    }
}

fn enrich_event(snapshot: &WorkspaceSnapshot, event: DomainEvent) -> DomainEvent {
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

#[no_mangle]
pub unsafe extern "C" fn argos_get_affected_scopes(
    workspace: *mut c_void,
    event_json: *const c_char,
    out_json: *mut *mut c_char,
) -> c_int {
    let handle = match as_handle(workspace) {
        Ok(h) => h,
        Err(e) => {
            set_last_error(&e);
            return -1;
        }
    };
    if event_json.is_null() {
        set_last_error("null event");
        return -1;
    }
    let raw = CStr::from_ptr(event_json).to_string_lossy();
    let event: DomainEvent = match serde_json::from_str(&raw) {
        Ok(e) => e,
        Err(e) => {
            set_last_error(&e.to_string());
            return -1;
        }
    };
    let snap = handle.workspace.current_snapshot();
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
    match serde_json::to_string(&affected) {
        Ok(j) => write_string(out_json, &j),
        Err(e) => {
            set_last_error(&e.to_string());
            -1
        }
    }
}

#[allow(dead_code)]
fn _silence_ptr() {
    let _ = ptr::null::<u8>();
}
