use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FileEventKind {
    FileCreated,
    FileModified,
    FileDeleted,
    FileRenamed,
    ScopeRescan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WorkspaceEventKind {
    WorkspaceUpdated,
    PackageAdded,
    PackageRemoved,
    TopologyChanged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "stream", rename_all = "camelCase")]
pub enum DomainEvent {
    File {
        kind: FileEventKind,
        path: String,
        node: Option<String>,
        scope: Option<String>,
    },
    Workspace {
        kind: WorkspaceEventKind,
        node: Option<String>,
        path: Option<String>,
    },
}
