# Design — m17 TypeScript napi-rs

## Decisions

| Question | Decision |
|----------|----------|
| Package name | `@doticca/argos` |
| Binding style | **napi-rs** crate `argos-napi` linking `argos-core` + OS backend (not ffi-dll) |
| Truth model | Snapshot JSON / typed projections only — no parallel runtime business state |
| Platforms | `win32-x64-msvc`, `linux-x64-gnu`, `darwin-arm64` |
| Packaging | napi-rs optional platform packages / prebuilds via `@napi-rs/cli` |
| Feed | GitHub Packages npm (`@doticca` scope) first; npmjs later |
| API style | Class `Workspace` mirroring .NET; prefer parsed objects over raw JSON strings |

## Architecture

```text
Node/TS app
  → @doticca/argos (JS + .d.ts)
  → argos.*.node (napi addon)
  → argos-core + argos-backend-{windows|linux|macos}
  → WorkspaceSnapshot (sole business truth)
```

Watch remains optional and scope-driven from the snapshot planner.

## Public API

| Method | Behavior |
|--------|----------|
| `Workspace.open(root, options?)` | Open workspace; returns handle |
| `currentSnapshot` | Latest immutable snapshot as object |
| `rebuildSnapshot()` | Rebuild; advance pointer |
| `listNodes()` / `listScopes()` | Query projections |
| `findOwner(path)` / `explainPath(path)` / `health()` | Diagnostics |
| `Workspace.computeDelta(a, b)` | Static delta |
| `watchStart` / `watchPoll` / `watchStop` | Native watch control |
| `watch(signal?)` | JS AsyncIterable over poll loop |
| `getAffectedScopes(event)` | Affected scope roots |
| `close()` / `[Symbol.asyncDispose]` | Release |

## Package / CI

- Crate: `crates/argos-napi` + package root `bindings/npm/`
- Prebuilds: `argos.win32-x64-msvc.node`, `argos.linux-x64-gnu.node`, `argos.darwin-arm64.node`
- Single npm package ships platform `.node` files selected by `binding.js` loader
- Main publish version: `{base}-ci.{GITHUB_RUN_NUMBER}` (npm semver; not NuGet 4-part)
- Workflow: `.github/workflows/pack-npm.yml` (path-filtered)

## Non-goals

See README. NuGet multi-RID stays separate.
