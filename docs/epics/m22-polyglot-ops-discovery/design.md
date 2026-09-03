# Design — polyglot + ops discovery

## Contract

Frozen architecture: new nodes in `WorkspaceSnapshot.model.nodes`; planner fills `planning.scopes`. No second public truth model.

## Discovery walk

Share skip-dir names with `nested_npm` / `dotnet`. Extend once in a shared helper if cheap; tests must fail if `vendor` or `.venv` leak as packages.

Order in `discover_topology` (after existing npm/cargo/dotnet, **before** document-root):

1. php-composer
2. python
3. go-mod / go-work
4. android-gradle
5. maven / gradle
6. dockerfile / compose / bicep / azure-yaml

Dedupe by `path_identity` as today. Ops **file** nodes use the file path as `root` (same as guidance-path files). Planner already supports file roots (`watch: ["."]`).

## PHP

- Node dir = parent of `composer.json`
- Confidence High
- Never walk into `vendor/`

## Python

- Prefer `pyproject.toml`, then `Pipfile`, then `requirements.txt` only at a **package dir** (sibling `*.py` or `src/`, not a docs-only stub)
- Never create a node whose root is `.venv` / `venv`

## Go

- One node per `go.mod` directory
- If `go.work` exists, add members as High-confidence nodes; leftover `go.mod` still fill-in

## Android / Gradle / Maven

- `settings.gradle(.kts)` → node at that dir
- Additional `build.gradle.kts` modules: fill-in if not the same path as a settings root
- `pom.xml` similarly; skip `target/`

## Ops

| Kind | `node.root` | Watch |
|------|-------------|--------|
| Dockerfile | file path | `"."` (the file) |
| Compose | file path | `"."` plus sibling dirs named in `services.*.build` if they exist (best-effort YAML; ignore parse failures) |
| Bicep | file path | `"."` |
| azure.yaml | file path | `"."` |

Compose full-tree watch is **out**; listed build contexts only.

## Intalepoint mapping

| Tree | Expected provider |
|------|-------------------|
| `service/intale-*.csproj` | existing `dotnet-*` |
| `packages/*/package.json` | existing `npm-nested` |
| `service/balena/**/Dockerfile` | `dockerfile` |
| `service/balena/docker-compose.yml` | `compose` |
| `service/intale-onewallet-android` | `android-gradle` |
| `**/composer.json` (not vendor) | `php-composer` |
| git submodule PHP apps | existing `git-submodule` **and** composer if checked out |
| `infra/**/*.bicep` | `bicep` |
| `azure.yaml` | `azure-yaml` |

## Tests

Unit tests per provider + one mixed fixture `fixtures/mixed-polyglot-ops` with php, py, go, Dockerfile, compose, one `.bicep`.
