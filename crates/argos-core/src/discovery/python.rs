use crate::model::{Confidence, DiscoverySource, NodeKind, WorkspaceNode};
use crate::Result;
use std::path::Path;

/// Discover Python packages via `pyproject.toml`, `Pipfile`, or package-root `requirements.txt`.
pub fn discover(root: &Path) -> Result<Vec<WorkspaceNode>> {
    let mut out = Vec::new();
    for path in super::walk::walk_entries(root) {
        let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        let Some(dir) = path.parent() else {
            continue;
        };
        let provider_manifest = match name {
            "pyproject.toml" => Some(("python", "pyproject.toml")),
            "Pipfile" if !dir.join("pyproject.toml").is_file() => Some(("python", "Pipfile")),
            "requirements.txt"
                if !dir.join("pyproject.toml").is_file()
                    && !dir.join("Pipfile").is_file()
                    && looks_like_python_package(dir) =>
            {
                Some(("python", "requirements.txt"))
            }
            _ => None,
        };
        let Some((provider, manifest)) = provider_manifest else {
            continue;
        };
        out.push(WorkspaceNode {
            id: super::walk::dir_id(dir, "python-package"),
            kind: NodeKind::Package,
            root: dir.to_path_buf(),
            path_identity: super::blank_identity(),
            source: DiscoverySource {
                provider: provider.into(),
                manifest: manifest.into(),
            },
            confidence: Confidence::High,
        });
    }
    Ok(out)
}

fn looks_like_python_package(dir: &Path) -> bool {
    if dir.join("src").is_dir() {
        return true;
    }
    let Ok(rd) = std::fs::read_dir(dir) else {
        return false;
    };
    rd.flatten().any(|e| {
        e.file_name()
            .to_string_lossy()
            .to_ascii_lowercase()
            .ends_with(".py")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn finds_pyproject_skips_venv() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("svc")).unwrap();
        fs::write(root.join("svc/pyproject.toml"), "[project]\nname='svc'\n").unwrap();
        fs::write(root.join("svc/main.py"), "print(1)\n").unwrap();
        fs::create_dir_all(root.join(".venv")).unwrap();
        fs::write(root.join(".venv/pyproject.toml"), "[project]\nname='venv'\n").unwrap();
        let nodes = discover(root).unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].id, "svc");
    }

    #[test]
    fn skips_docs_only_requirements() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(root.join("docs/requirements.txt"), "sphinx\n").unwrap();
        let nodes = discover(root).unwrap();
        assert!(nodes.is_empty());
    }

    #[test]
    fn accepts_requirements_with_py_sibling() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(root.join("requirements.txt"), "flask\n").unwrap();
        fs::write(root.join("app.py"), "").unwrap();
        let nodes = discover(root).unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].source.manifest, "requirements.txt");
    }
}
