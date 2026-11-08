//! Consumer path include/exclude filters for filesystem mode.

use globset::{Glob, GlobSet, GlobSetBuilder};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathFilterSpec {
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PathFilter {
    include: Option<GlobSet>,
    exclude: Option<GlobSet>,
    include_patterns: Vec<String>,
    exclude_patterns: Vec<String>,
    case_sensitive: bool,
}

impl PathFilter {
    pub fn empty(case_sensitive: bool) -> Self {
        Self {
            include: None,
            exclude: None,
            include_patterns: Vec::new(),
            exclude_patterns: Vec::new(),
            case_sensitive,
        }
    }

    pub fn from_spec(spec: &PathFilterSpec, case_sensitive: bool) -> crate::Result<Self> {
        let include = build_set(&spec.include)?;
        let exclude = build_set(&spec.exclude)?;
        Ok(Self {
            include,
            exclude,
            include_patterns: spec.include.clone(),
            exclude_patterns: spec.exclude.clone(),
            case_sensitive,
        })
    }

    pub fn spec(&self) -> PathFilterSpec {
        PathFilterSpec {
            include: self.include_patterns.clone(),
            exclude: self.exclude_patterns.clone(),
        }
    }

    /// Relative path from the registered root, using `/` separators.
    pub fn normalize_rel(path: &Path) -> String {
        path.to_string_lossy()
            .replace('\\', "/")
            .trim_start_matches('/')
            .to_string()
    }

    fn candidate(&self, rel: &str) -> String {
        if self.case_sensitive {
            rel.to_string()
        } else {
            rel.to_lowercase()
        }
    }

    /// Whether a directory should be traversed while discovering matching descendants.
    /// Exclude that matches the directory itself blocks traversal; include never blocks traversal.
    pub fn should_traverse_dir(&self, rel: &str) -> bool {
        let c = self.candidate(rel);
        if let Some(ex) = &self.exclude {
            if !c.is_empty() && ex.is_match(&c) {
                return false;
            }
            // Also match trailing-slash style directory patterns against `rel/**` form.
            let as_tree = if c.is_empty() {
                String::new()
            } else {
                format!("{c}/")
            };
            if !as_tree.is_empty() && ex.is_match(&as_tree) {
                return false;
            }
        }
        true
    }

    /// File eligibility: exclude wins; empty include means all files are eligible.
    pub fn file_matches(&self, rel: &str) -> bool {
        let c = self.candidate(rel);
        if let Some(ex) = &self.exclude {
            if ex.is_match(&c) {
                return false;
            }
        }
        match &self.include {
            None => true,
            Some(inc) => inc.is_match(&c),
        }
    }

    /// Directory events always pass filters so consumers can reconcile structure.
    pub fn directory_event_passes(&self) -> bool {
        true
    }
}

fn build_set(patterns: &[String]) -> crate::Result<Option<GlobSet>> {
    if patterns.is_empty() {
        return Ok(None);
    }
    let mut builder = GlobSetBuilder::new();
    for p in patterns {
        let normalized = p.replace('\\', "/");
        let glob = Glob::new(&normalized)
            .map_err(|e| crate::ArgosError::Parse(format!("filter glob '{normalized}': {e}")))?;
        builder.add(glob);
    }
    let set = builder
        .build()
        .map_err(|e| crate::ArgosError::Parse(format!("filter globset: {e}")))?;
    Ok(Some(set))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exclude_wins_over_include() {
        let filter = PathFilter::from_spec(
            &PathFilterSpec {
                include: vec!["**/*.flac".into()],
                exclude: vec!["**/*.partial".into(), "staging/**".into()],
            },
            true,
        )
        .unwrap();
        assert!(filter.file_matches("Album/track.flac"));
        assert!(!filter.file_matches("Album/track.flac.partial"));
        assert!(!filter.file_matches("staging/a.flac"));
        assert!(filter.should_traverse_dir("Album"));
        assert!(!filter.should_traverse_dir("staging"));
    }

    #[test]
    fn empty_include_allows_all() {
        let filter = PathFilter::from_spec(
            &PathFilterSpec {
                include: vec![],
                exclude: vec!["tmp/**".into()],
            },
            true,
        )
        .unwrap();
        assert!(filter.file_matches("a.mp3"));
        assert!(!filter.file_matches("tmp/x"));
    }
}
