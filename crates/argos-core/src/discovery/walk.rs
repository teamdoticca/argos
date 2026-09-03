use walkdir::WalkDir;
use std::path::{Path, PathBuf};

/// Directories never entered during topology walks.
pub const SKIP_DIR_NAMES: &[&str] = &[
    ".git",
    "node_modules",
    "dist",
    "build",
    "target",
    "coverage",
    ".next",
    ".turbo",
    ".cache",
    "bin",
    "obj",
    ".vs",
    "artifacts",
    "vendor",
    ".venv",
    "venv",
    "__pycache__",
    ".tox",
    ".mypy_cache",
    "Pods",
];

pub fn is_skip_dir_name(name: &str) -> bool {
    SKIP_DIR_NAMES
        .iter()
        .any(|s| name.eq_ignore_ascii_case(s))
}

pub fn walk_entries(root: &Path) -> impl Iterator<Item = PathBuf> {
    WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            if !e.file_type().is_dir() {
                return true;
            }
            let name = e.file_name().to_string_lossy();
            !is_skip_dir_name(&name)
        })
        .flatten()
        .map(|e| e.into_path())
}

pub fn dir_id(dir: &Path, fallback: &str) -> String {
    dir.file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| fallback.into())
}

pub fn file_id(path: &Path, fallback: &str) -> String {
    path.file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| fallback.into())
}
