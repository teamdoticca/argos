//! Argos core — workspace intelligence engine.
//!
//! `WorkspaceSnapshot` is the system contract and sole business truth.

mod delta;
mod diagnostics;
mod discovery;
mod error;
mod events;
mod ignore_engine;
mod model;
mod path_identity;
mod planner;
mod snapshot;
mod workspace;

pub use delta::{compute_delta, SnapshotDelta};
pub use diagnostics::{
    explain_path, explain_scope, find_owner, list_nodes, list_scopes, platform_info, snapshot_validate,
    workspace_health, ExplainPathResult, ExplainScopeResult, HealthReport, OwnerHit, PlatformInfo,
    SnapshotValidation, Stats, ValidationIssue,
};
pub use discovery::discover_topology;
pub use error::{ArgosError, Result};
pub use events::{DomainEvent, FileEventKind, WorkspaceEventKind};
pub use ignore_engine::{hard_default_names, IgnoreEngine};
pub use model::{
    Confidence, DiscoverySource, NodeKind, TopologyResult, WorkspaceNode,
};
pub use path_identity::{detect_case_sensitivity, PathIdentity, PathSemantics};
pub use planner::{absolute_watch_paths, apply_planner, plan_scopes, WatchScope};
pub use snapshot::{
    FilesystemSection, IdentitySection, ModelSection, PlanningSection, SnapshotState,
    WorkspaceSnapshot, SCHEMA_VERSION,
};
pub use workspace::{OpenOptions, Workspace};
