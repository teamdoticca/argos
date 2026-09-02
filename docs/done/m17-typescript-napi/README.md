# First-class TypeScript (napi-rs)

**Epic:** `m17-typescript-napi`  
**Status:** see `status.md`

## Goal

Ship **`@teamdoticca/argos`** as a first-class Node/TypeScript package via **napi-rs**, linking `argos-core` + platform backends directly (not a JS loader over `argos_ffi`). Mirror the .NET `ArgosWorkspace` surface. Natives for **win32-x64-msvc**, **linux-x64-gnu**, and **darwin-arm64**. `WorkspaceSnapshot` remains sole business truth.

**One-line outcome:** a clean Node project can `npm install @teamdoticca/argos`, `Workspace.open(root)`, get non-empty scopes on fixtures, and watch planned paths on Windows, Linux, and Apple Silicon.

## Acceptance criteria

- [x] Design documents package name, napi vs ffi-dll, three-platform packaging, typed snapshot contract
- [x] Local win-x64: `Workspace.open(fixture)` → ready snapshot, scopes > 0; watch smoke ≥1 event
- [x] CI workflow matrix: win-x64, linux-x64, darwin-arm64 (`.github/workflows/pack-npm.yml`)
- [x] Clean Node project load path documented (README + `@teamdoticca/argos`)
- [x] No second public truth model beside `WorkspaceSnapshot`
- [x] Root README consumer Node section (no CI internals)
- [x] `docs/roadmap.md` and `docs/execution-plan.md` updated on progress
- [x] CI green on all three OS runners + publish to GitHub Packages and npmjs.org
- [x] On completion, move to `docs/done/m17-typescript-napi`

## Out of scope

- linux-arm64 / darwin-x64 / musl
- Python bindings
- Discovery/planner changes (unless smoke finds a bug)
- Shared daemon / Watchman
- NuGet RID matrix expansion
- Replacing Mnemon’s .NET PackageReference
