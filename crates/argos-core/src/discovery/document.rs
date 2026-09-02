//! Discover document roots and guidance-cited important paths.
//!
//! - `document-root`: scored planning/docs trees (not a fixed `docs/` name)
//! - `guidance-path`: folders/files named in agent guidance that packages miss

use crate::model::{Confidence, DiscoverySource, NodeKind, WorkspaceNode};
use crate::Result;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

const SKIP_DIR_NAMES: &[&str] = &[
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
];

const DOC_NAME_ALIASES: &[&str] = &[
    "docs",
    "documentation",
    "doc",
    "handbook",
    "wiki",
    "guides",
    "guide",
    "adr",
    "adrs",
    "rfcs",
    "rfc",
    "specs",
    "spec",
    "epics",
];

const PLANNING_NAME_HINTS: &[&str] = &[
    "roadmap",
    "execution-plan",
    "architecture",
    "agents",
    "contributing",
    "adr-",
    "adr_",
];

const DOC_EXTENSIONS: &[&str] = &["md", "mdx", "adoc", "rst"];
const CODE_EXTENSIONS: &[&str] = &[
    "ts", "tsx", "js", "jsx", "rs", "cs", "py", "go", "java", "kt", "cpp", "c", "h",
];

const MAX_GUIDANCE_BYTES: usize = 64 * 1024;
const WALK_MAX_DEPTH: usize = 3;
const SCORE_THRESHOLD: i32 = 15;

/// Discover document-root and guidance-path nodes.
///
/// `existing` is the topology discovered so far (packages, etc.) for dedupe.
pub fn discover(root: &Path, existing: &[WorkspaceNode]) -> Result<Vec<WorkspaceNode>> {
    let package_roots: Vec<PathBuf> = existing
        .iter()
        .filter(|n| {
            matches!(
                n.kind,
                NodeKind::Package | NodeKind::GitSubmodule | NodeKind::ImportedFolder
            ) && n.source.provider != "fallback-root"
                && n.source.provider != "heuristic"
        })
        .map(|n| n.root.clone())
        .collect();

    let (guidance_hits, opened_guidance) = collect_guidance_hits(root)?;

    let mut guidance_dirs: HashSet<PathBuf> = HashSet::new();
    let mut root_files: Vec<PathBuf> = Vec::new();

    for hit in &guidance_hits {
        if hit.is_dir() {
            if hit != root {
                guidance_dirs.insert(hit.clone());
            }
            continue;
        }
        if let Some(parent) = hit.parent() {
            if parent == root {
                root_files.push(hit.clone());
            } else if parent != root {
                guidance_dirs.insert(parent.to_path_buf());
            }
        }
    }

    // Primary agent guidance files we opened are important surfaces (not README/CONTRIBUTING).
    for g in &opened_guidance {
        if !is_primary_agent_guidance(g) {
            continue;
        }
        if g.parent().is_some_and(|p| p == root) {
            if !root_files.iter().any(|f| f == g) {
                root_files.push(g.clone());
            }
        } else if let Some(parent) = g.parent() {
            if parent != root {
                guidance_dirs.insert(parent.to_path_buf());
            }
        }
    }

    let mut candidates: HashMap<PathBuf, bool> = HashMap::new();
    for d in walk_candidate_dirs(root) {
        candidates.entry(d).or_insert(false);
    }
    for d in &guidance_dirs {
        candidates.insert(d.clone(), true);
    }

    let mut scored: Vec<(PathBuf, i32, String)> = Vec::new();
    for (dir, from_guidance) in candidates {
        if is_junk_path(root, &dir) {
            continue;
        }
        if covered_by_package(&dir, &package_roots) {
            // Still allow a docs-like tree that happens to sit beside packages;
            // only skip when the candidate *is* a package root or lives under package src-ish.
            if package_roots.iter().any(|p| p == &dir) {
                continue;
            }
            // Under a package: only keep if strong doc alias or guidance pointed here.
            let under_pkg = package_roots.iter().any(|p| dir.starts_with(p));
            if under_pkg && !from_guidance && !name_is_doc_alias(&dir) {
                continue;
            }
        }
        let (score, manifest) = score_document_dir(&dir, from_guidance);
        if score >= SCORE_THRESHOLD {
            scored.push((dir, score, manifest));
        }
    }

    scored.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let mut out = Vec::new();
    let mut emitted_roots: HashSet<PathBuf> = HashSet::new();
    let mut doc_count = 0usize;
    for (dir, score, manifest) in scored {
        if doc_count >= 2 {
            break;
        }
        // Skip if nested under an already chosen stronger document root.
        if emitted_roots.iter().any(|r| dir.starts_with(r) && dir != *r) {
            continue;
        }
        let conf = confidence_from_score(score);
        out.push(make_node(
            root,
            &dir,
            "document-root",
            &manifest,
            conf,
            NodeKind::Package,
        ));
        emitted_roots.insert(dir);
        doc_count += 1;
    }

    // Guidance directories not chosen as document-root and not package-covered.
    let mut guidance_dir_list: Vec<PathBuf> = guidance_dirs.into_iter().collect();
    guidance_dir_list.sort();
    for dir in guidance_dir_list {
        if emitted_roots.contains(&dir) {
            continue;
        }
        if dir == root {
            continue;
        }
        if is_junk_path(root, &dir) {
            continue;
        }
        if package_roots.iter().any(|p| p == &dir || dir.starts_with(p)) {
            // Allow agent surfaces under repo even when a fallback package owns root —
            // only skip when under a *real* package root that isn't the workspace root.
            let under_real = package_roots.iter().any(|p| p != root && dir.starts_with(p));
            if under_real {
                continue;
            }
        }
        let manifest = dir
            .strip_prefix(root)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|_| dir.to_string_lossy().replace('\\', "/"));
        out.push(make_node(
            root,
            &dir,
            "guidance-path",
            &manifest,
            Confidence::High,
            NodeKind::Package,
        ));
        emitted_roots.insert(dir);
    }

    for file in root_files {
        if !file.is_file() {
            continue;
        }
        // Identity is the file path so multiple root files do not collapse.
        let rel = file
            .strip_prefix(root)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|_| {
                file.file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "file".into())
            });
        out.push(WorkspaceNode {
            id: format!("guidance:{}", rel.replace('/', "-")),
            kind: NodeKind::Package,
            root: file.clone(),
            path_identity: crate::discovery::blank_identity(),
            source: DiscoverySource {
                provider: "guidance-path".into(),
                manifest: rel,
            },
            confidence: Confidence::High,
        });
    }

    Ok(out)
}

fn make_node(
    workspace: &Path,
    dir: &Path,
    provider: &str,
    manifest: &str,
    confidence: Confidence,
    kind: NodeKind,
) -> WorkspaceNode {
    let rel = dir
        .strip_prefix(workspace)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| {
            dir.file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| provider.into())
        });
    let id = if provider == "document-root" {
        format!("docs:{}", rel.replace('/', "-"))
    } else {
        format!("guidance:{}", rel.replace('/', "-"))
    };
    WorkspaceNode {
        id,
        kind,
        root: dir.to_path_buf(),
        path_identity: crate::discovery::blank_identity(),
        source: DiscoverySource {
            provider: provider.into(),
            manifest: manifest.into(),
        },
        confidence,
    }
}

fn confidence_from_score(score: i32) -> Confidence {
    if score >= 40 {
        Confidence::High
    } else if score >= 25 {
        Confidence::Medium
    } else {
        Confidence::Low
    }
}

fn name_is_doc_alias(dir: &Path) -> bool {
    dir.file_name()
        .and_then(|s| s.to_str())
        .map(|n| DOC_NAME_ALIASES.iter().any(|a| n.eq_ignore_ascii_case(a)))
        .unwrap_or(false)
}

fn covered_by_package(path: &Path, packages: &[PathBuf]) -> bool {
    packages.iter().any(|p| path == p || path.starts_with(p))
}

fn is_junk_path(root: &Path, path: &Path) -> bool {
    let Ok(rel) = path.strip_prefix(root) else {
        return true;
    };
    for c in rel.components() {
        let name = c.as_os_str().to_string_lossy();
        if SKIP_DIR_NAMES
            .iter()
            .any(|s| name.eq_ignore_ascii_case(s))
        {
            return true;
        }
    }
    false
}

fn walk_candidate_dirs(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let walker = walkdir::WalkDir::new(root)
        .follow_links(false)
        .max_depth(WALK_MAX_DEPTH)
        .into_iter()
        .filter_entry(|e| {
            if !e.file_type().is_dir() {
                return true;
            }
            let name = e.file_name().to_string_lossy();
            !SKIP_DIR_NAMES.iter().any(|s| name.eq_ignore_ascii_case(s))
        });
    for entry in walker.flatten() {
        let path = entry.path();
        if !path.is_dir() || path == root {
            continue;
        }
        if is_junk_path(root, path) {
            continue;
        }
        out.push(path.to_path_buf());
    }
    out
}

fn score_document_dir(dir: &Path, from_guidance: bool) -> (i32, String) {
    let mut score: i32 = 0;
    let mut best_manifest = dir
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "dir".into());

    if name_is_doc_alias(dir) {
        score += 5;
    }

    let mut md = 0i32;
    let mut code = 0i32;
    let mut planning = 0i32;

    let walker = walkdir::WalkDir::new(dir)
        .follow_links(false)
        .max_depth(2)
        .into_iter()
        .filter_entry(|e| {
            if !e.file_type().is_dir() {
                return true;
            }
            let name = e.file_name().to_string_lossy();
            !SKIP_DIR_NAMES.iter().any(|s| name.eq_ignore_ascii_case(s))
        });

    for entry in walker.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name_lower = path
            .file_name()
            .map(|s| s.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        let ext = path
            .extension()
            .and_then(|x| x.to_str())
            .unwrap_or("")
            .to_lowercase();

        if DOC_EXTENSIONS.iter().any(|e| *e == ext) {
            md += 1;
            if PLANNING_NAME_HINTS
                .iter()
                .any(|h| name_lower.contains(h))
            {
                planning += 1;
                best_manifest = path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or(best_manifest.clone());
            }
        } else if CODE_EXTENSIONS.iter().any(|e| *e == ext) {
            code += 1;
        }
    }

    // Guidance boost only for trees that already look like documents.
    // Agent surfaces (`.cursor/rules`, workflows, …) stay as `guidance-path`.
    let looks_like_docs = md > 0 || planning > 0 || name_is_doc_alias(dir);
    if from_guidance && looks_like_docs {
        score += 25;
    }

    score += md.saturating_mul(3).min(30);
    score += planning.saturating_mul(15);
    if code > md.saturating_mul(2) && md < 3 {
        score -= 20;
    }

    (score, best_manifest)
}

fn collect_guidance_hits(root: &Path) -> Result<(Vec<PathBuf>, Vec<PathBuf>)> {
    let mut opened = Vec::new();
    let mut texts: Vec<String> = Vec::new();

    let root_names = [
        "AGENTS.md",
        "agents.md",
        "CLAUDE.md",
        "GEMINI.md",
        ".cursorrules",
        "CONTRIBUTING.md",
        "README.md",
    ];
    for name in root_names {
        let p = root.join(name);
        if p.is_file() {
            if let Some(t) = read_bounded(&p) {
                texts.push(t);
                opened.push(p);
            }
        }
    }

    let rules_dir = root.join(".cursor/rules");
    if rules_dir.is_dir() {
        if let Ok(rd) = std::fs::read_dir(&rules_dir) {
            for e in rd.flatten().take(40) {
                let path = e.path();
                if path.is_file() {
                    if let Some(t) = read_bounded(&path) {
                        texts.push(t);
                        opened.push(path);
                    }
                }
            }
        }
    }

    let mut hits = Vec::new();
    let mut seen = HashSet::new();
    for text in texts {
        for raw in extract_path_tokens(&text) {
            let resolved = resolve_repo_path(root, &raw);
            if let Some(p) = resolved {
                if seen.insert(p.clone()) {
                    hits.push(p);
                }
            }
        }
    }
    Ok((hits, opened))
}

fn read_bounded(path: &Path) -> Option<String> {
    let data = std::fs::read(path).ok()?;
    if data.is_empty() {
        return None;
    }
    let slice = if data.len() > MAX_GUIDANCE_BYTES {
        &data[..MAX_GUIDANCE_BYTES]
    } else {
        &data
    };
    Some(String::from_utf8_lossy(slice).into_owned())
}

fn extract_path_tokens(text: &str) -> Vec<String> {
    let mut out = Vec::new();

    // Backtick segments.
    let mut parts = text.split('`');
    let _ = parts.next(); // outside
    for (i, seg) in parts.enumerate() {
        if i % 2 == 0 {
            // inside backticks
            let t = seg.trim();
            if looks_like_repo_path(t) {
                out.push(t.to_string());
            }
        }
    }

    // Markdown links: ](path)
    let bytes = text.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b']' && bytes[i + 1] == b'(' {
            let start = i + 2;
            if let Some(rel) = text[start..].find(')') {
                let inner = text[start..start + rel].trim();
                let path_part = inner.split_whitespace().next().unwrap_or(inner);
                let path_part = path_part.split('#').next().unwrap_or(path_part);
                if looks_like_repo_path(path_part) {
                    out.push(path_part.to_string());
                }
                i = start + rel;
                continue;
            }
        }
        i += 1;
    }

    out
}

fn looks_like_repo_path(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() || s.len() > 260 {
        return false;
    }
    if s.contains('\n') || s.contains(' ') || s.contains('<') || s.contains('>') {
        return false;
    }
    let lower = s.to_ascii_lowercase();
    if lower.starts_with("http://")
        || lower.starts_with("https://")
        || lower.starts_with("mailto:")
        || lower.starts_with("data:")
    {
        return false;
    }
    // Reject bare skill/rule ids without a path separator.
    let has_sep = s.contains('/') || s.contains('\\');
    let starts_dot = s.starts_with('.');
    let has_ext = Path::new(s).extension().is_some();
    has_sep || (starts_dot && has_ext) || (has_ext && s.contains('.'))
}

fn resolve_repo_path(root: &Path, raw: &str) -> Option<PathBuf> {
    let cleaned = raw.trim().trim_start_matches("./");
    if cleaned.is_empty() || cleaned.starts_with("..") {
        return None;
    }
    let candidate = root.join(cleaned);
    if candidate.exists() {
        Some(candidate)
    } else {
        None
    }
}

fn is_primary_agent_guidance(path: &Path) -> bool {
    let name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if matches!(
        name.as_str(),
        "agents.md" | "claude.md" | "gemini.md" | ".cursorrules"
    ) {
        return true;
    }
    // Files under .cursor/rules
    path.components().any(|c| {
        c.as_os_str()
            .to_str()
            .is_some_and(|s| s.eq_ignore_ascii_case("rules"))
    }) && path
        .components()
        .any(|c| {
            c.as_os_str()
                .to_str()
                .is_some_and(|s| s.eq_ignore_ascii_case(".cursor"))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write(path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, content).unwrap();
    }

    #[test]
    fn scores_handbook_not_only_docs_name() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write(&root.join("handbook/roadmap.md"), "# Roadmap\n");
        write(&root.join("handbook/architecture.md"), "# Arch\n");
        let nodes = discover(root, &[]).unwrap();
        assert!(
            nodes.iter().any(|n| {
                n.source.provider == "document-root" && n.root.ends_with("handbook")
            }),
            "got {:?}",
            nodes
                .iter()
                .map(|n| (&n.source.provider, &n.root))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn guidance_points_to_odd_tree_and_rules() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write(
            &root.join("AGENTS.md"),
            "Read `knowledge-base/intro.md` and `.cursor/rules/x.mdc`.\n",
        );
        write(&root.join("knowledge-base/intro.md"), "# Intro\n");
        write(&root.join("knowledge-base/roadmap.md"), "# Roadmap\n");
        write(&root.join(".cursor/rules/x.mdc"), "rule\n");

        let nodes = discover(root, &[]).unwrap();
        assert!(
            nodes.iter().any(|n| {
                n.source.provider == "document-root" && n.root.ends_with("knowledge-base")
            }),
            "expected knowledge-base document-root, got {:?}",
            nodes
                .iter()
                .map(|n| (&n.id, &n.source.provider))
                .collect::<Vec<_>>()
        );
        assert!(
            nodes.iter().any(|n| {
                n.source.provider == "guidance-path" && n.root.ends_with("rules")
            }),
            "expected .cursor/rules guidance-path"
        );
        assert!(nodes.iter().any(|n| {
            n.source.provider == "guidance-path" && n.root.ends_with("AGENTS.md")
        }));
    }

    #[test]
    fn skips_node_modules_docs() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write(
            &root.join("node_modules/pkg/docs/readme.md"),
            "# Vendor\n",
        );
        write(&root.join("node_modules/pkg/docs/roadmap.md"), "# R\n");
        let nodes = discover(root, &[]).unwrap();
        assert!(
            !nodes.iter().any(|n| n.source.provider == "document-root"),
            "must not pick node_modules docs"
        );
    }

    #[test]
    fn does_not_duplicate_package_root() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write(
            &root.join("AGENTS.md"),
            "See `packages/app` and `docs/roadmap.md`.\n",
        );
        write(&root.join("docs/roadmap.md"), "# R\n");
        write(&root.join("packages/app/package.json"), r#"{"name":"app"}"#);
        write(&root.join("packages/app/src/index.ts"), "export {}");

        let existing = vec![WorkspaceNode {
            id: "app".into(),
            kind: NodeKind::Package,
            root: root.join("packages/app"),
            path_identity: crate::discovery::blank_identity(),
            source: DiscoverySource {
                provider: "pnpm".into(),
                manifest: "package.json".into(),
            },
            confidence: Confidence::High,
        }];
        let nodes = discover(root, &existing).unwrap();
        assert!(nodes.iter().any(|n| n.source.provider == "document-root"));
        assert!(!nodes.iter().any(|n| n.root.ends_with("app")
            && n.source.provider == "guidance-path"));
    }
}
