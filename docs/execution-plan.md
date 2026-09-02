# Argos execution plan

Live status. Update on every meaningful progress.

## Current

- **Active epic:** none (m00–m14 complete)
- **Status:** done
- **Last update:** 2026-09-02 — product implementation complete through m14

## Completed

### m00-foundation (done)

- Local repo, GitHub, English docs, agents/rules/skills

### m01–m06 Workspace Intelligence (done)

- Topology discovery (pnpm/npm/yarn/cargo/gitmodules/`.workspace` imports-only + OS artifacts)
- PathIdentity, case sensitivity, symlink/junction helpers
- Immutable `WorkspaceSnapshot` sections + versions + state
- Ignore engine (hard defaults + nested gitignore / `.ignore` / `.cursorignore`)
- Scope planner (known + discoverable roots; no recursive monorepo-root watch)
- Diagnostics/query APIs + rebuild_snapshot + validate + health

### m07–m12 Change Tracking platform (done)

- `WatchBackend` + `PlatformCapabilities`
- Windows RDCW / Linux inotify / macOS FSEvents backends (via `notify`)
- File Events + Workspace Events + `compute_delta` (content-version semantics)
- Overflow → recovering → scoped rescan recovery

### m13–m14 Packaging & benchmarks (done)

- NuGet `Argos` C# wrapper over C ABI (`argos-ffi`)
- `argos-bench` harness + `fixtures/small-pnpm`

## Next

1. Harden CI across win/linux/macos RIDs
2. Pack and publish NuGet when ready
3. Mnemon integration spike (reference consumer)

## Blockers

None.
