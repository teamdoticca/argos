# Polyglot + ops topology discovery

**Epic:** `m22-polyglot-ops-discovery`
**Status:** done

## Goal

Open → Snapshot → Scope Planner must produce useful watch scopes for **PHP, Python, Go, Android/Gradle, JVM**, and **ops manifests** (Dockerfile, Compose, Bicep, `azure.yaml`) so Focused consumers (Mnemon) see dirties in those trees — not only npm / .NET / Cargo.

Reference layout: mixed product trees such as Intalepoint (.NET + npm + PHP submodules + Android + balena Docker + Azure infra).

Depends on **m21** (package-root `"."` planner) so new flat language packages inherit that watch behavior.

## Why

Today discovery is npm, nested `package.json`, Cargo, `.sln`/`.csproj`, git submodules, `.workspace` imports, document/guidance. Python / PHP / Go have **no first-class providers**. `pyproject.toml` is only a planner config name if a node already exists. Docker / Bicep never become nodes.

Ops files are **high signal** (device images, compose stacks, Azure Bicep). They are in scope for this epic, not deferred.

## Slices (implement in order)

### 0 — Ignore / walk skip (do first)

Hard-skip on discovery walks **and** ignore defaults:

- `vendor` (Composer)
- `.venv`, `venv`, `__pycache__`, `.tox`, `.mypy_cache`
- `Pods` (CocoaPods)
- keep existing: `node_modules`, `bin`, `obj`, `target`, `dist`, `build`, `.git`, …

Without this, Composer/Python discovery will ingest junk.

### 1 — P0 languages

| Provider | Manifest | Skip |
|----------|----------|------|
| `php-composer` | `composer.json` | dirs named `vendor` |
| `python` | `pyproject.toml`, `Pipfile`, package-root `requirements.txt` | `.venv` / `venv` |
| `go-mod` | `go.mod` | — |
| `go-work` | `go.work` members | — |

Planner: reuse package-root `"."` when sources live next to the manifest (m21).

### 2 — P1 Android / Gradle

| Provider | Manifest |
|----------|----------|
| `android-gradle` | `settings.gradle` / `settings.gradle.kts`; `build.gradle.kts` app/library modules |

Example: `service/intale-onewallet-android`.

### 3 — P2 JVM

| Provider | Manifest |
|----------|----------|
| `maven` | `pom.xml` (skip `target`) |
| `gradle` | `build.gradle` / `build.gradle.kts` not already claimed by Android |

### 4 — Ops manifests (required)

Each file (or compose project) is a node. Watch the **file**; for Compose, also **immediate sibling build-context dirs that exist**. Do not recurse the whole git root.

| Provider | Manifest |
|----------|----------|
| `dockerfile` | `Dockerfile`, `Dockerfile.*` |
| `compose` | `docker-compose.yml`, `docker-compose.yaml`, `compose.yml`, `compose.yaml` |
| `bicep` | `*.bicep` (file-targeted node, like guidance-path files) |
| `azure-yaml` | `azure.yaml` (azd) |

Prefer **keeping ops nodes** even when a parent package already watches a directory, so Focused still lists the Dockerfile / `.bicep`.

## Acceptance criteria

- [x] Slice 0 skip/ignore list shipped; Composer `vendor` and Python `.venv` never become packages
- [x] PHP: `composer.json` → node (`fixtures/mixed-polyglot-ops/php-app`; `vendor` skipped)
- [x] Python: `pyproject.toml` / `Pipfile` / package-root `requirements.txt` → node; `.venv` ignored
- [x] Go: `go.mod` (+ `go.work` members) → nodes
- [x] Android: gradle/android node with non-empty watch (fixture)
- [x] Maven/Gradle JVM fixtures → nodes (no explosion from nested `build/`)
- [x] Dockerfile + compose + `.bicep` + `azure.yaml` → nodes; watch includes those files
- [x] Fixtures under `fixtures/mixed-polyglot-ops`
- [x] Argos self-repo + `fixtures/small-pnpm` still green
- [x] Smoke: Open on mixed fixture → `ListScopes` includes new providers
- [x] `docs/roadmap.md` and `docs/execution-plan.md` updated on progress
- [x] Public **Argos 0.1.7** — nuget.org + GitHub Packages; npm `0.1.7`
- [x] Moved to `docs/done/m22-polyglot-ops-discovery`

## Out of scope

- Language-aware indexing / symbols
- Nx / Turborepo
- Watching entire Docker build contexts as the git root
- Helm / Terraform (follow-up)
- Live Intalepoint smoke / Mnemon PackageReference bump (separate repo, after ship)
