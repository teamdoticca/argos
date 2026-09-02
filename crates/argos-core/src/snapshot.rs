use crate::model::WorkspaceNode;
use crate::path_identity::PathSemantics;
use crate::planner::WatchScope;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SnapshotState {
    Ready,
    Degraded,
    Recovering,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentitySection {
    pub workspace_id: Uuid,
    pub schema_version: u32,
    pub snapshot_version: u64,
    pub previous_snapshot_version: Option<u64>,
    pub content_version: u64,
    pub previous_content_version: Option<u64>,
    pub state: SnapshotState,
    pub root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelSection {
    pub nodes: Vec<WorkspaceNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilesystemSection {
    pub case_sensitive: bool,
    pub symlink_mode: String,
    pub semantics: PathSemantics,
    pub artifacts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanningSection {
    pub ignores: Vec<String>,
    pub scopes: Vec<WatchScope>,
}

/// Immutable workspace snapshot — system contract / sole business truth.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSnapshot {
    pub identity: IdentitySection,
    pub model: ModelSection,
    pub filesystem: FilesystemSection,
    pub planning: PlanningSection,
}

impl WorkspaceSnapshot {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
