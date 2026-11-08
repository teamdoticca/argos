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
        /// Prior path when a reliable rename is reported.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from_path: Option<String>,
        /// New path when a reliable rename is reported.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to_path: Option<String>,
        /// Recovery / uncertainty reason (`overflow`, `root-unavailable`, …).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reason: Option<String>,
    },
    Workspace {
        kind: WorkspaceEventKind,
        node: Option<String>,
        path: Option<String>,
    },
}

impl DomainEvent {
    pub fn file(
        kind: FileEventKind,
        path: impl Into<String>,
        node: Option<String>,
        scope: Option<String>,
    ) -> Self {
        DomainEvent::File {
            kind,
            path: path.into(),
            node,
            scope,
            from_path: None,
            to_path: None,
            reason: None,
        }
    }

    pub fn file_with_reason(
        kind: FileEventKind,
        path: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        DomainEvent::File {
            kind,
            path: path.into(),
            node: None,
            scope: None,
            from_path: None,
            to_path: None,
            reason: Some(reason.into()),
        }
    }

    pub fn file_renamed(from: impl Into<String>, to: impl Into<String>) -> Self {
        let from_path = from.into();
        let to_path = to.into();
        DomainEvent::File {
            kind: FileEventKind::FileRenamed,
            path: to_path.clone(),
            node: None,
            scope: None,
            from_path: Some(from_path),
            to_path: Some(to_path),
            reason: None,
        }
    }
}
