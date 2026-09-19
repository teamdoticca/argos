use crate::path_identity::PathIdentity;
use globset::{Glob, GlobSet, GlobSetBuilder};
use serde::{Deserialize, Serialize};
use std::path::Path;

const HARD_DEFAULTS: &[&str] = &[
    ".git",
    "node_modules",
    "dist",
    "build",
    "target",
    "coverage",
    ".next",
    ".turbo",
    ".cache",
    "vendor",
    ".venv",
    "venv",
    "__pycache__",
    ".tox",
    ".mypy_cache",
    "Pods",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IgnoreRule {
    pub pattern: String,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct IgnoreEngine {
    pub rules: Vec<IgnoreRule>,
    set: GlobSet,
    case_sensitive: bool,
}

impl IgnoreEngine {
    pub fn build(root: &Path, case_sensitive: bool) -> crate::Result<Self> {
        let mut rules = Vec::new();
        for d in HARD_DEFAULTS {
            rules.push(IgnoreRule {
                pattern: format!("**/{d}/**"),
                source: "hard-default".into(),
            });
            rules.push(IgnoreRule {
                pattern: format!("**/{d}"),
                source: "hard-default".into(),
            });
        }

        collect_ignore_file(root, ".gitignore", "gitignore", &mut rules);
        collect_ignore_file(root, ".ignore", "ignore", &mut rules);
        collect_ignore_file(root, ".cursorignore", "cursorignore", &mut rules);
        // Nested gitignores (shallow walk, skip hard defaults)
        collect_nested_gitignores(root, &mut rules);

        let mut builder = GlobSetBuilder::new();
        for r in &rules {
            if let Ok(g) = Glob::new(&r.pattern) {
                builder.add(g);
            }
        }
        let set = builder
            .build()
            .map_err(|e| crate::ArgosError::Parse(format!("globset: {e}")))?;

        Ok(Self {
            rules,
            set,
            case_sensitive,
        })
    }

    pub fn is_ignored(&self, path: &Path) -> bool {
        let rel = path.to_string_lossy().replace('\\', "/");
        let candidate = if self.case_sensitive {
            rel
        } else {
            rel.to_lowercase()
        };
        self.set.is_match(candidate)
    }

    pub fn explain(&self, path: &Path) -> Option<&IgnoreRule> {
        let rel = path.to_string_lossy().replace('\\', "/");
        let candidate = if self.case_sensitive {
            rel.clone()
        } else {
            rel.to_lowercase()
        };
        if !self.set.is_match(&candidate) {
            return None;
        }
        // Return first matching rule by re-checking patterns (approximate).
        for r in &self.rules {
            if let Ok(g) = Glob::new(&r.pattern) {
                if g.compile_matcher().is_match(&candidate) {
                    return Some(r);
                }
            }
        }
        self.rules.first()
    }

    pub fn rule_patterns(&self) -> Vec<String> {
        self.rules.iter().map(|r| r.pattern.clone()).collect()
    }

    pub fn matches_identity(&self, id: &PathIdentity) -> bool {
        self.is_ignored(Path::new(&id.display))
    }
}

fn collect_ignore_file(root: &Path, name: &str, source: &str, rules: &mut Vec<IgnoreRule>) {
    let path = root.join(name);
    let Ok(text) = std::fs::read_to_string(path) else {
        return;
    };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('!') {
            continue;
        }
        let pattern = if line.contains('/') || line.contains('*') {
            if line.starts_with('/') {
                format!("**{}", line)
            } else if line.ends_with('/') {
                format!("**/{line}**")
            } else {
                format!("**/{line}")
            }
        } else {
            format!("**/{line}/**")
        };
        rules.push(IgnoreRule {
            pattern,
            source: source.into(),
        });
    }
}

fn collect_nested_gitignores(root: &Path, rules: &mut Vec<IgnoreRule>) {
    let walker = walkdir::WalkDir::new(root).max_depth(4).into_iter();
    for entry in walker.flatten() {
        let path = entry.path();
        if path.file_name().and_then(|s| s.to_str()) != Some(".gitignore") {
            continue;
        }
        if path.parent() == Some(root) {
            continue;
        }
        let Some(parent) = path.parent() else {
            continue;
        };
        let Ok(rel) = parent.strip_prefix(root) else {
            continue;
        };
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        let prefix = rel.to_string_lossy().replace('\\', "/");
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('!') {
                continue;
            }
            rules.push(IgnoreRule {
                pattern: format!("**/{prefix}/{line}"),
                source: format!("gitignore:{prefix}"),
            });
        }
    }
}

pub fn hard_default_names() -> &'static [&'static str] {
    HARD_DEFAULTS
}
