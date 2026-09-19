use crate::snapshot::WorkspaceSnapshot;
use crate::ArgosError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotDelta {
    pub added_nodes: Vec<String>,
    pub removed_nodes: Vec<String>,
    pub changed_scopes: Vec<String>,
    pub content_changed: bool,
}

/// Compute delta using content-version semantics.
/// Rebuild-without-change (same contentVersion) yields empty business delta.
pub fn compute_delta(
    old: &WorkspaceSnapshot,
    new: &WorkspaceSnapshot,
) -> crate::Result<SnapshotDelta> {
    if old.identity.workspace_id != new.identity.workspace_id {
        return Err(ArgosError::InvalidDelta("workspaceId mismatch".into()));
    }
    if new.identity.previous_content_version != Some(old.identity.content_version)
        && new.identity.content_version != old.identity.content_version
    {
        // Allow equal content versions (no business change) or consecutive content versions.
        if new.identity.content_version != old.identity.content_version + 1
            && !(new.identity.content_version == old.identity.content_version
                && new.identity.previous_content_version == old.identity.previous_content_version)
        {
            return Err(ArgosError::InvalidDelta(format!(
                "non-consecutive content versions: {} -> {}",
                old.identity.content_version, new.identity.content_version
            )));
        }
    }

    if new.identity.content_version == old.identity.content_version {
        return Ok(SnapshotDelta {
            added_nodes: vec![],
            removed_nodes: vec![],
            changed_scopes: vec![],
            content_changed: false,
        });
    }

    let old_ids: std::collections::HashSet<_> =
        old.model.nodes.iter().map(|n| n.id.clone()).collect();
    let new_ids: std::collections::HashSet<_> =
        new.model.nodes.iter().map(|n| n.id.clone()).collect();

    let added_nodes: Vec<_> = new_ids.difference(&old_ids).cloned().collect();
    let removed_nodes: Vec<_> = old_ids.difference(&new_ids).cloned().collect();

    let mut changed_scopes = Vec::new();
    for scope in &new.planning.scopes {
        let old_scope = old
            .planning
            .scopes
            .iter()
            .find(|s| s.package == scope.package);
        match old_scope {
            None => changed_scopes.push(scope.package.clone()),
            Some(o) if o.watch != scope.watch || o.root != scope.root => {
                changed_scopes.push(scope.package.clone())
            }
            _ => {}
        }
    }

    Ok(SnapshotDelta {
        added_nodes,
        removed_nodes,
        changed_scopes,
        content_changed: true,
    })
}
