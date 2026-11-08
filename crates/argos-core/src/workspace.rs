use crate::diagnostics::{platform_info, PlatformInfo};
use crate::discovery::{
    discover_filesystem_root, discover_topology, SymlinkPolicy,
};
use crate::ignore_engine::IgnoreEngine;
use crate::path_filter::{PathFilter, PathFilterSpec};
use crate::path_identity::{detect_case_sensitivity, PathSemantics};
use crate::planner::plan_scopes;
use crate::snapshot::{
    CoverageReport, FilesystemSection, IdentitySection, ModelSection, PlanningSection,
    SnapshotState, WorkspaceSnapshot, SCHEMA_VERSION,
};
use crate::{ArgosError, Result};
use parking_lot::RwLock;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use uuid::Uuid;

pub const DEFAULT_FILESYSTEM_EVENT_BUFFER: usize = 8192;

#[derive(Debug, Clone, Default)]
pub struct OpenOptions {
    /// `"workspace"` (default) or `"filesystem"`.
    pub mode: Option<String>,
    pub symlink_mode: Option<String>,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub event_buffer_capacity: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscoveryMode {
    Workspace,
    Filesystem,
}

impl DiscoveryMode {
    pub fn parse(raw: Option<&str>) -> Result<Self> {
        match raw.map(str::trim).unwrap_or("workspace") {
            "" | "workspace" => Ok(Self::Workspace),
            "filesystem" => Ok(Self::Filesystem),
            other => Err(ArgosError::Parse(format!(
                "unknown open mode '{other}'; expected 'workspace' or 'filesystem'"
            ))),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Workspace => "workspace",
            Self::Filesystem => "filesystem",
        }
    }
}

/// Consumer-owned workspace handle. `current_snapshot` is a pointer to an immutable snapshot.
pub struct Workspace {
    root: PathBuf,
    workspace_id: Uuid,
    case_sensitive: bool,
    mode: DiscoveryMode,
    symlink_mode: String,
    symlink_policy: SymlinkPolicy,
    ignore: IgnoreEngine,
    path_filter: PathFilter,
    event_buffer_capacity: usize,
    snapshot: RwLock<Arc<WorkspaceSnapshot>>,
    snapshot_version: RwLock<u64>,
    content_version: RwLock<u64>,
    backend_name: RwLock<Option<String>>,
}

impl Workspace {
    pub fn open(root: impl AsRef<Path>, options: OpenOptions) -> Result<Self> {
        let root = root.as_ref();
        if !root.exists() {
            return Err(ArgosError::RootMissing(root.display().to_string()));
        }
        if !root.is_dir() {
            return Err(ArgosError::RootNotDirectory(root.display().to_string()));
        }
        let root = crate::path_identity::canonicalize_lossy(root);
        let case_sensitive = detect_case_sensitivity(&root);
        let mode = DiscoveryMode::parse(options.mode.as_deref())?;
        let symlink_mode = resolve_symlink_mode(mode, options.symlink_mode.as_deref())?;
        let symlink_policy = SymlinkPolicy::parse(&symlink_mode).ok_or_else(|| {
            ArgosError::Parse(format!("unknown symlinkMode '{symlink_mode}'"))
        })?;
        let workspace_id = Uuid::new_v4();
        let filter_spec = PathFilterSpec {
            include: options.include.unwrap_or_default(),
            exclude: options.exclude.unwrap_or_default(),
        };
        let path_filter = PathFilter::from_spec(&filter_spec, case_sensitive)?;
        let ignore = match mode {
            DiscoveryMode::Workspace => IgnoreEngine::build(&root, case_sensitive)?,
            DiscoveryMode::Filesystem => IgnoreEngine::empty(case_sensitive)?,
        };
        let event_buffer_capacity = options
            .event_buffer_capacity
            .unwrap_or(match mode {
                DiscoveryMode::Filesystem => DEFAULT_FILESYSTEM_EVENT_BUFFER,
                DiscoveryMode::Workspace => DEFAULT_FILESYSTEM_EVENT_BUFFER,
            })
            .max(1);

        let (snapshot, snap_v, content_v) = build_snapshot(
            &root,
            workspace_id,
            case_sensitive,
            mode,
            &symlink_mode,
            symlink_policy,
            &ignore,
            &path_filter,
            1,
            1,
            None,
            None,
            SnapshotState::Ready,
        )?;

        Ok(Self {
            root,
            workspace_id,
            case_sensitive,
            mode,
            symlink_mode,
            symlink_policy,
            ignore,
            path_filter,
            event_buffer_capacity,
            snapshot: RwLock::new(Arc::new(snapshot)),
            snapshot_version: RwLock::new(snap_v),
            content_version: RwLock::new(content_v),
            backend_name: RwLock::new(None),
        })
    }

    pub fn workspace_id(&self) -> Uuid {
        self.workspace_id
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn mode(&self) -> DiscoveryMode {
        self.mode
    }

    pub fn event_buffer_capacity(&self) -> usize {
        self.event_buffer_capacity
    }

    pub fn path_filter(&self) -> &PathFilter {
        &self.path_filter
    }

    pub fn symlink_policy(&self) -> SymlinkPolicy {
        self.symlink_policy
    }

    /// Pointer to the latest immutable snapshot.
    pub fn current_snapshot(&self) -> Arc<WorkspaceSnapshot> {
        self.snapshot.read().clone()
    }

    pub fn rebuild_snapshot(&self) -> Result<Arc<WorkspaceSnapshot>> {
        if !self.root.exists() {
            return Err(ArgosError::RootMissing(self.root.display().to_string()));
        }
        let prev_snap = *self.snapshot_version.read();
        let prev_content = *self.content_version.read();
        let old = self.current_snapshot();
        let (mut new_snap, _, _) = build_snapshot(
            &self.root,
            self.workspace_id,
            self.case_sensitive,
            self.mode,
            &self.symlink_mode,
            self.symlink_policy,
            &self.ignore,
            &self.path_filter,
            prev_snap + 1,
            prev_content,
            Some(prev_snap),
            Some(prev_content),
            SnapshotState::Ready,
        )?;

        // Bump content version only when business content changes.
        let content_changed = content_fingerprint(&old) != content_fingerprint(&new_snap);
        if content_changed {
            new_snap.identity.content_version = prev_content + 1;
            new_snap.identity.previous_content_version = Some(prev_content);
            *self.content_version.write() = prev_content + 1;
        } else {
            new_snap.identity.content_version = prev_content;
            new_snap.identity.previous_content_version = old.identity.previous_content_version;
        }
        *self.snapshot_version.write() = prev_snap + 1;
        let arc = Arc::new(new_snap);
        *self.snapshot.write() = arc.clone();
        Ok(arc)
    }

    pub fn ignore(&self) -> &IgnoreEngine {
        &self.ignore
    }

    pub fn set_backend_name(&self, name: Option<String>) {
        *self.backend_name.write() = name;
    }

    pub fn platform_info(&self) -> PlatformInfo {
        let mut info = platform_info(self.case_sensitive, self.backend_name.read().clone());
        info.symlink_mode = self.symlink_mode.clone();
        info
    }

    pub fn mark_recovering(&self) -> Result<Arc<WorkspaceSnapshot>> {
        let mut snap = (*self.current_snapshot()).clone();
        let prev = snap.identity.snapshot_version;
        snap.identity.previous_snapshot_version = Some(prev);
        snap.identity.snapshot_version = prev + 1;
        snap.identity.state = SnapshotState::Recovering;
        snap.planning.coverage.requires_reconciliation = true;
        if !snap
            .planning
            .coverage
            .notes
            .iter()
            .any(|n| n == "recovery")
        {
            snap.planning.coverage.notes.push("recovery".into());
        }
        *self.snapshot_version.write() = snap.identity.snapshot_version;
        let arc = Arc::new(snap);
        *self.snapshot.write() = arc.clone();
        Ok(arc)
    }

    pub fn mark_ready_from_rebuild(&self) -> Result<Arc<WorkspaceSnapshot>> {
        let snap = self.rebuild_snapshot()?;
        Ok(snap)
    }

    pub fn mark_degraded_root_unavailable(&self, reason: &str) -> Result<Arc<WorkspaceSnapshot>> {
        let mut snap = (*self.current_snapshot()).clone();
        let prev = snap.identity.snapshot_version;
        snap.identity.previous_snapshot_version = Some(prev);
        snap.identity.snapshot_version = prev + 1;
        snap.identity.state = SnapshotState::Degraded;
        snap.planning.coverage.complete = false;
        snap.planning.coverage.requires_reconciliation = true;
        snap.planning.coverage.notes.push(reason.into());
        *self.snapshot_version.write() = snap.identity.snapshot_version;
        let arc = Arc::new(snap);
        *self.snapshot.write() = arc.clone();
        Ok(arc)
    }

    /// Whether a relative-to-root path should emit a file event under current filters.
    pub fn file_event_eligible(&self, absolute: &Path) -> bool {
        if self.mode != DiscoveryMode::Filesystem {
            return true;
        }
        let absolute = crate::path_identity::canonicalize_lossy(absolute);
        let Ok(rel) = absolute.strip_prefix(&self.root) else {
            // Outside registered root: not eligible as a filtered library file event.
            return false;
        };
        let rel = PathFilter::normalize_rel(rel);
        if absolute.is_dir() {
            return self.path_filter.directory_event_passes();
        }
        // If path no longer exists (delete), still apply filter against the path string.
        self.path_filter.file_matches(&rel)
    }
}

fn resolve_symlink_mode(mode: DiscoveryMode, raw: Option<&str>) -> Result<String> {
    match (mode, raw.map(str::trim).filter(|s| !s.is_empty())) {
        (DiscoveryMode::Workspace, None) => Ok("follow".into()),
        (DiscoveryMode::Filesystem, None) => Ok("nofollow".into()),
        (_, Some(s)) => {
            if SymlinkPolicy::parse(s).is_none() {
                return Err(ArgosError::Parse(format!(
                    "unknown symlinkMode '{s}'; expected follow, nofollow, or followConfined"
                )));
            }
            // Normalize follow-confined alias.
            Ok(SymlinkPolicy::parse(s).unwrap().as_str().to_string())
        }
    }
}

#[allow(
    clippy::too_many_arguments,
    reason = "Keeps snapshot identity and version inputs explicit at the construction boundary"
)]
fn build_snapshot(
    root: &Path,
    workspace_id: Uuid,
    case_sensitive: bool,
    mode: DiscoveryMode,
    symlink_mode: &str,
    symlink_policy: SymlinkPolicy,
    ignore: &IgnoreEngine,
    path_filter: &PathFilter,
    snapshot_version: u64,
    content_version: u64,
    previous_snapshot_version: Option<u64>,
    previous_content_version: Option<u64>,
    state: SnapshotState,
) -> Result<(WorkspaceSnapshot, u64, u64)> {
    let (topology, coverage) = match mode {
        DiscoveryMode::Workspace => {
            let topology = discover_topology(root)?;
            (
                topology,
                CoverageReport {
                    complete: true,
                    unreadable: Vec::new(),
                    skipped_links: Vec::new(),
                    skipped_outside_root: Vec::new(),
                    notes: Vec::new(),
                    requires_reconciliation: false,
                },
            )
        }
        DiscoveryMode::Filesystem => {
            let discovered =
                discover_filesystem_root(root, case_sensitive, symlink_policy, path_filter)?;
            (discovered.topology, discovered.coverage)
        }
    };

    let mut warnings_degraded = false;
    for n in &topology.nodes {
        if !n.root.exists() {
            warnings_degraded = true;
            break;
        }
    }
    let state = if (warnings_degraded || !coverage.complete)
        && matches!(state, SnapshotState::Ready)
        && mode == DiscoveryMode::Filesystem
        && (!coverage.unreadable.is_empty())
    {
        SnapshotState::Degraded
    } else if warnings_degraded && matches!(state, SnapshotState::Ready) {
        SnapshotState::Degraded
    } else {
        state
    };

    let scopes = plan_scopes(&topology.nodes, ignore);
    let artifacts: Vec<String> = topology
        .nodes
        .iter()
        .filter(|n| matches!(n.kind, crate::model::NodeKind::OsArtifact))
        .map(|n| format!("{}:{}", n.source.manifest, n.path_identity.display))
        .collect();
    let filters = match mode {
        DiscoveryMode::Filesystem => path_filter.spec(),
        DiscoveryMode::Workspace => PathFilterSpec::default(),
    };
    let snap = WorkspaceSnapshot {
        identity: IdentitySection {
            workspace_id,
            schema_version: SCHEMA_VERSION,
            snapshot_version,
            previous_snapshot_version,
            content_version,
            previous_content_version,
            state,
            root: root.to_string_lossy().replace('\\', "/"),
            mode: mode.as_str().into(),
        },
        model: ModelSection {
            nodes: topology.nodes,
        },
        filesystem: FilesystemSection {
            case_sensitive,
            symlink_mode: symlink_mode.to_string(),
            semantics: PathSemantics {
                case_sensitive,
                follows_symlinks: symlink_policy.follows_symlinks(),
                symlink_policy: symlink_mode.to_string(),
            },
            artifacts,
        },
        planning: PlanningSection {
            ignores: ignore.rule_patterns(),
            scopes,
            filters,
            coverage,
        },
    };
    Ok((snap, snapshot_version, content_version))
}

fn content_fingerprint(snapshot: &WorkspaceSnapshot) -> String {
    let mut nodes: Vec<_> = snapshot
        .model
        .nodes
        .iter()
        .map(|n| {
            format!(
                "{}:{:?}:{}",
                n.id,
                n.kind,
                n.root.to_string_lossy().replace('\\', "/")
            )
        })
        .collect();
    nodes.sort();
    let mut scopes: Vec<_> = snapshot
        .planning
        .scopes
        .iter()
        .map(|s| format!("{}:{}:{:?}", s.package, s.root, s.watch))
        .collect();
    scopes.sort();
    let coverage = format!(
        "{}:{}:{}",
        snapshot.planning.coverage.complete,
        snapshot.planning.coverage.requires_reconciliation,
        snapshot.planning.coverage.unreadable.len()
    );
    format!("{nodes:?}|{scopes:?}|{coverage}")
}
