use serde::{Deserialize, Serialize};
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathSemantics {
    pub case_sensitive: bool,
    pub follows_symlinks: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PathIdentity {
    /// Canonical absolute path string used for equality.
    pub key: String,
    pub display: String,
}

impl PathIdentity {
    pub fn from_path(path: &Path, case_sensitive: bool) -> Self {
        let canonical = canonicalize_lossy(path);
        let display = canonical.to_string_lossy().replace('\\', "/");
        let key = if case_sensitive {
            display.clone()
        } else {
            display.to_lowercase()
        };
        Self { key, display }
    }

    pub fn equals(&self, other: &PathIdentity) -> bool {
        self.key == other.key
    }
}

pub fn detect_case_sensitivity(root: &Path) -> bool {
    // Probe: on case-insensitive FS, creating/opening differently cased names collide.
    // Default heuristic by platform; refined when probe file can be written.
    if cfg!(windows) {
        false
    } else if cfg!(target_os = "macos") {
        // Often case-insensitive by default (APFS can vary); treat as insensitive unless proven.
        !probe_case_sensitive(root)
    } else {
        true
    }
}

fn probe_case_sensitive(root: &Path) -> bool {
    let probe = root.join(".argos_case_probe_UPPER");
    let lower = root.join(".argos_case_probe_upper");
    let _ = std::fs::remove_file(&probe);
    let _ = std::fs::remove_file(&lower);
    if std::fs::write(&probe, b"x").is_err() {
        return !cfg!(windows);
    }
    let sensitive = !lower.exists() || probe.canonicalize().ok() != lower.canonicalize().ok();
    let _ = std::fs::remove_file(&probe);
    let _ = std::fs::remove_file(&lower);
    sensitive
}

pub fn canonicalize_lossy(path: &Path) -> PathBuf {
    if let Ok(c) = path.canonicalize() {
        // Strip Windows \\?\ prefix for stable display.
        let s = c.to_string_lossy();
        if let Some(stripped) = s.strip_prefix(r"\\?\") {
            return PathBuf::from(stripped);
        }
        return c;
    }
    normalize_lexically(path)
}

fn normalize_lexically(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for c in path.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

pub fn classify_symlink(path: &Path) -> Option<String> {
    let meta = std::fs::symlink_metadata(path).ok()?;
    if meta.file_type().is_symlink() {
        Some("symlink".into())
    } else {
        None
    }
}

#[cfg(windows)]
pub fn is_junction_or_reparse(path: &Path) -> bool {
    use std::os::windows::fs::MetadataExt;
    let Ok(meta) = std::fs::symlink_metadata(path) else {
        return false;
    };
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
    (meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT) != 0
}

#[cfg(not(windows))]
pub fn is_junction_or_reparse(_path: &Path) -> bool {
    false
}
