# Design — m17 TypeScript napi-rs

## Decisions

| Question | Decision |
|----------|----------|
| Package name | `@teamdoticca/argos` (matches GitHub org; required for GitHub Packages npm) |
| Binding style | **napi-rs** crate `argos-napi` linking `argos-core` + OS backend (not ffi-dll) |
| Truth model | Snapshot JSON / typed projections only — no parallel runtime business state |
| Platforms | `win32-x64-msvc`, `linux-x64-gnu`, `darwin-arm64` |
| Packaging | Single tarball with three `.node` natives + `binding.js` (not split platform packages) |
| Feed | GitHub Packages for CI dogfood; npmjs.org for public install (explicit publish) — see [npmjs-prep.md](./npmjs-prep.md) |
| API style | Class `Workspace` mirroring .NET; prefer parsed objects over raw JSON strings |

## Architecture

```text
Node/TS app
  → @teamdoticca/argos (JS + .d.ts)
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
- GitHub Packages: `{base}-ci.{GITHUB_RUN_NUMBER}` on main (path-filtered)
- npmjs.org: **workflow_dispatch only** (`registry=npmjs|both`) + secret `NPM_TOKEN`; release semver from `package.json`
- Checklist: [npmjs-prep.md](./npmjs-prep.md)
- Workflow: `.github/workflows/pack-npm.yml`

## Non-goals

See README. NuGet multi-RID stays separate.
