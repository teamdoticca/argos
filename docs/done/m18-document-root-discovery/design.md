# Design — m18 document root discovery

## Problem

Discovery is package-centric. The scope planner’s conventional dirs (`src`, `app`, `lib`, …) do not include documentation trees or agent-era guidance surfaces. Important paths mentioned in `AGENTS.md` (docs trees, `.cursor/rules`, skills, workflows, config files, …) never become nodes or watch targets → `WatchAsync` misses churn that agents treat as first-class. That is wrong for agent-era monorepos.

## Non-goal: fixed `docs/`

**Do not** special-case “if `docs/` exists, watch it.” Many repos use other names; some have a decorative `docs/` that is not the planning truth. The initial scan must **find** important document directory(ies) by scoring evidence, and must **also** surface other important folders/files called out by agent guidance — not markdown-only.

## Two outputs from one scan

| Output | Provider | What it is |
|--------|----------|------------|
| Document root(s) | `document-root` | Scored planning/docs tree(s) |
| Guidance-important paths | `guidance-path` | Extra folders **or files** referenced in agent guidance that exist on disk and are not already covered by package topology |

Both land as topology nodes (or scoped watch entries projected from the snapshot). No second public truth model.

## Provider A: `document-root`

Ignore-aware. Does not replace package nodes.

### Candidate collection

- Shallow walk from the workspace root (bounded depth; skip ignored paths and known junk: `node_modules`, `target`, `.git`, build outputs).
- Candidates are **directories**.
- Prefer directories that are not already interior to a discovered package’s primary source layout when scores would otherwise tie.
- **Also seed** from guidance-extracted directory paths (below).

### Scoring signals (examples — tune in implementation)

**Positive**

| Signal | Role |
|--------|------|
| Markdown / MDX / AsciiDoc / RST density | Primary content evidence for *document* roots |
| Planning anchors on disk (`roadmap`, `execution-plan`, `architecture`, ADR-like names, epic folders) | Strong planning-tree boost |
| Paths referenced inside agent guidance files | Strong seed / boost when the referenced tree exists |
| Directory name aliases (`docs`, `documentation`, `handbook`, `wiki`, `guides`, `adr`, `rfcs`, `specs`, …) | Soft boost only — never required |

**Negative**

| Signal | Role |
|--------|------|
| Dominant source-code extensions | Penalize (package `src`, not docs) |
| Inside dependency / build trees | Disqualify or hard penalty |
| Extremely deep / huge vendor-like trees | Cap or skip |

### Selection

1. Build candidate set (walk + guidance-seeded dirs).
2. Score; keep above threshold.
3. Prefer shallower / higher-score; usually **one primary** document root; optional second if clearly independent.
4. Score bands → `Confidence`.
5. Weak evidence → **no** `document-root` node (do not invent; do not fall back to “always `docs/`”).

## Agent guidance files (read for hints)

Do **not** treat `AGENTS.md` (and peers) as filename-only anchors. During the initial scan, **open and parse** well-known agent / contributor guidance files and use their **content** as discovery input for **both** document roots and other important paths.

**Files to consider (existence + bounded read):**

| File / pattern | Why |
|----------------|-----|
| `AGENTS.md` / `agents.md` | Operating map for agents: docs, rules, skills, crates, workflows |
| `CLAUDE.md`, `GEMINI.md`, `.cursorrules`, `.cursor/rules/*` | Same class |
| `CONTRIBUTING.md` (optional weak) | Sometimes lists layout |
| Root `README.md` (bounded; optional) | May link docs / contributing layout |

**How to use content (heuristic, not LLM):**

1. Bounded read (size/line cap) — never load huge blobs.
2. Extract repo-relative path-like tokens and markdown links that resolve under the workspace — **any extension or directory**, not only `.md` (examples: `docs/roadmap.md`, `.cursor/rules/`, `.cursor/skills/`, `AGENTS.md`, `.github/workflows/`, `crates/argos-core/`, `bindings/npm/`).
3. Classify each existing hit:
   - **Directory** → candidate for `document-root` scoring *and/or* `guidance-path` node
   - **File** → `guidance-path` watch target (file scope) **or** boost parent dir for document-root scoring when the file looks like planning/docs
4. Strong boost when multiple guidance files agree on the same path.
5. Pointers never invent missing paths: **must exist on disk**.

## Provider B: `guidance-path`

Surfaces important folders/files that guidance named explicitly, beyond the single document root.

### Emit when

- Path was extracted from guidance content, and
- Path exists, ignore-aware, and
- Path is **not redundant** with an existing package / submodule / import node root (or already fully covered by that node’s planned watch), and
- Path is not junk (`node_modules`, build outputs, `.git`).

### Prefer / allow examples

- Agent surfaces: `.cursor/rules`, `.cursor/skills`, root `AGENTS.md`
- Automation: `.github/workflows` when referenced
- Config / contract files called out by name
- Oddly named planning dirs already handled via document-root; if they win there, do not double-emit

### Avoid absorbing whole codebases

If guidance mentions a package path that discovery already owns as a `Package` node, **do not** add a duplicate `guidance-path` for that package root. Guidance is for gaps the package graph misses (docs, rules, CI, lone root files), not a second package enumerator.

### Node / watch shape

- `source.provider`: `guidance-path`
- `root`: directory containing the target, or the file’s parent with watch entry naming the file — implementation may emit directory nodes with explicit file watch lists
- Confidence: typically `High` when repeatedly cited; `Medium` for a single clear path hit

### Scope planner

- `document-root`: watch `"."` under that tree (ignore-aware).
- `guidance-path`: watch the cited directory (`"."`) or the cited file relative to its parent.
- Do not require `KNOWN_DIRS` (`src`/`app`) for these providers.

Optional later: auto-watch root `AGENTS.md` even when unreferenced (presence-only). Prefer guidance-cited + document-root wins in this epic.

## Snapshot truth

All of the above are topology / planning projections of `WorkspaceSnapshot`. No second public truth model.

## Safe boundaries

- Still **scoped** watch — not recursive whole-repo happy path.
- Package discovery regressions must stay green.
- Weak document evidence → no `document-root`; guidance may still emit narrower `guidance-path` hits.
- Heuristic path extract only — no LLM interpretation of prose.
