//! Filesystem-only topology: one registered root, no software discovery.

use crate::model::{Confidence, DiscoverySource, NodeKind, TopologyResult, WorkspaceNode};
use crate::path_filter::PathFilter;
use crate::path_identity::{
    canonicalize_lossy, classify_symlink, is_path_within_root, PathIdentity,
};
use crate::snapshot::CoverageReport;
use crate::Result;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

const MAX_COVERAGE_DEPTH: usize = 64;
const MAX_COVERAGE_ENTRIES: usize = 50_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymlinkPolicy {
    NoFollow,
    FollowConfined,
    Follow,
}

impl SymlinkPolicy {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "nofollow" => Some(Self::NoFollow),
            "followConfined" | "follow-confined" => Some(Self::FollowConfined),
            "follow" => Some(Self::Follow),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoFollow => "nofollow",
            Self::FollowConfined => "followConfined",
            Self::Follow => "follow",
        }
    }

    pub fn follows_symlinks(self) -> bool {
        !matches!(self, Self::NoFollow)
    }
}

pub struct FilesystemDiscovery {
    pub topology: TopologyResult,
    pub coverage: CoverageReport,
}

pub fn discover_filesystem_root(
    root: &Path,
    case_sensitive: bool,
    symlink_policy: SymlinkPolicy,
    filter: &PathFilter,
) -> Result<FilesystemDiscovery> {
    // Root must be readable as a directory; permission errors are hard failures.
    let meta = std::fs::metadata(root).map_err(|e| {
        if e.kind() == std::io::ErrorKind::PermissionDenied {
            crate::ArgosError::Message(format!(
                "filesystem root is not readable: {}: {e}",
                root.display()
            ))
        } else {
            crate::ArgosError::Io(e)
        }
    })?;
    if !meta.is_dir() {
        return Err(crate::ArgosError::RootNotDirectory(root.display().to_string()));
    }
    // Prove we can list the root; do not pretend an unreadable root is empty.
    let _ = std::fs::read_dir(root).map_err(|e| {
        crate::ArgosError::Message(format!(
            "filesystem root cannot be listed: {}: {e}",
            root.display()
        ))
    })?;

    let root_canon = canonicalize_lossy(root);
    let mut coverage = CoverageReport {
        complete: true,
        unreadable: Vec::new(),
        skipped_links: Vec::new(),
        skipped_outside_root: Vec::new(),
        notes: vec!["startup-gap".into()],
        requires_reconciliation: true,
    };

    walk_coverage(
        &root_canon,
        &root_canon,
        case_sensitive,
        symlink_policy,
        filter,
        &mut coverage,
        &mut HashSet::new(),
        0,
        &mut 0,
    )?;

    if !coverage.unreadable.is_empty()
        || !coverage.skipped_links.is_empty()
        || !coverage.skipped_outside_root.is_empty()
    {
        coverage.complete = false;
    }

    let node = WorkspaceNode {
        id: "filesystem-root".into(),
        kind: NodeKind::Package,
        root: root_canon.clone(),
        path_identity: PathIdentity::from_path(&root_canon, case_sensitive),
        source: DiscoverySource {
            provider: "filesystem-root".into(),
            manifest: ".".into(),
        },
        confidence: Confidence::High,
    };

    Ok(FilesystemDiscovery {
        topology: TopologyResult { nodes: vec![node] },
        coverage,
    })
}

#[allow(
    clippy::too_many_arguments,
    reason = "Coverage walk keeps policy inputs explicit"
)]
fn walk_coverage(
    root: &Path,
    dir: &Path,
    case_sensitive: bool,
    symlink_policy: SymlinkPolicy,
    filter: &PathFilter,
    coverage: &mut CoverageReport,
    visited: &mut HashSet<String>,
    depth: usize,
    entries: &mut usize,
) -> Result<()> {
    if depth > MAX_COVERAGE_DEPTH || *entries >= MAX_COVERAGE_ENTRIES {
        coverage.complete = false;
        coverage.notes.push("coverage-walk-bounded".into());
        return Ok(());
    }

    let dir_id = PathIdentity::from_path(dir, case_sensitive);
    if !visited.insert(dir_id.key.clone()) {
        return Ok(());
    }

    let rd = match std::fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(e) => {
            coverage.complete = false;
            coverage.unreadable.push(format!("{} ({e})", display_rel(root, dir)));
            return Ok(());
        }
    };

    for entry in rd {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                coverage.complete = false;
                coverage
                    .unreadable
                    .push(format!("{} ({e})", display_rel(root, dir)));
                continue;
            }
        };
        *entries += 1;
        if *entries >= MAX_COVERAGE_ENTRIES {
            coverage.complete = false;
            coverage.notes.push("coverage-walk-bounded".into());
            break;
        }

        let path = entry.path();
        let rel = match path.strip_prefix(root) {
            Ok(r) => PathFilter::normalize_rel(r),
            Err(_) => PathFilter::normalize_rel(&path),
        };

        let ft = match entry.file_type() {
            Ok(ft) => ft,
            Err(e) => {
                coverage.complete = false;
                coverage.unreadable.push(format!("{rel} ({e})"));
                continue;
            }
        };

        if ft.is_symlink() || classify_symlink(&path).is_some() {
            handle_symlink(
                root,
                &path,
                &rel,
                case_sensitive,
                symlink_policy,
                filter,
                coverage,
                visited,
                depth,
                entries,
            )?;
            continue;
        }

        if ft.is_dir() {
            if !filter.should_traverse_dir(&rel) {
                continue;
            }
            walk_coverage(
                root,
                &path,
                case_sensitive,
                symlink_policy,
                filter,
                coverage,
                visited,
                depth + 1,
                entries,
            )?;
        }
    }
    Ok(())
}

#[allow(
    clippy::too_many_arguments,
    reason = "Symlink policy handling keeps containment inputs explicit"
)]
fn handle_symlink(
    root: &Path,
    link_path: &Path,
    rel: &str,
    case_sensitive: bool,
    symlink_policy: SymlinkPolicy,
    filter: &PathFilter,
    coverage: &mut CoverageReport,
    visited: &mut HashSet<String>,
    depth: usize,
    entries: &mut usize,
) -> Result<()> {
    match symlink_policy {
        SymlinkPolicy::NoFollow => {
            coverage.skipped_links.push(rel.to_string());
            coverage.complete = false;
            Ok(())
        }
        SymlinkPolicy::Follow => {
            // Explicit unconfined follow: traverse if target is a directory; diagnose limitation.
            if !coverage.notes.iter().any(|n| n == "symlink-follow-unconfined") {
                coverage.notes.push("symlink-follow-unconfined".into());
            }
            let target = match std::fs::canonicalize(link_path) {
                Ok(t) => strip_unc(t),
                Err(e) => {
                    coverage.complete = false;
                    coverage
                        .skipped_links
                        .push(format!("{rel} (unresolvable: {e})"));
                    return Ok(());
                }
            };
            if target.is_dir() && filter.should_traverse_dir(rel) {
                walk_coverage(
                    root,
                    &target,
                    case_sensitive,
                    symlink_policy,
                    filter,
                    coverage,
                    visited,
                    depth + 1,
                    entries,
                )?;
            }
            Ok(())
        }
        SymlinkPolicy::FollowConfined => {
            let target = match std::fs::canonicalize(link_path) {
                Ok(t) => strip_unc(t),
                Err(e) => {
                    coverage.complete = false;
                    coverage
                        .skipped_links
                        .push(format!("{rel} (unresolvable: {e})"));
                    return Ok(());
                }
            };
            if !is_path_within_root(&target, root, case_sensitive) {
                coverage.complete = false;
                coverage.skipped_outside_root.push(rel.to_string());
                return Ok(());
            }
            if target.is_dir() && filter.should_traverse_dir(rel) {
                walk_coverage(
                    root,
                    &target,
                    case_sensitive,
                    symlink_policy,
                    filter,
                    coverage,
                    visited,
                    depth + 1,
                    entries,
                )?;
            }
            Ok(())
        }
    }
}

fn display_rel(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .map(PathFilter::normalize_rel)
        .unwrap_or_else(|_| path.to_string_lossy().replace('\\', "/"))
}

fn strip_unc(path: PathBuf) -> PathBuf {
    let s = path.to_string_lossy();
    if let Some(stripped) = s.strip_prefix(r"\\?\") {
        PathBuf::from(stripped)
    } else {
        path
    }
}
