# Argos execution plan

Live status. Update on every meaningful progress.

## Current

- **Active epic:** none
- **Next planned:** `m15-nuget-pack-ci` (pack + CI for consumable NuGet)
- **Status:** planned (docs ready; implementation not started)
- **Last update:** 2026-09-02 — added m15 NuGet pack & CI planning epic

## Planned

### m15-nuget-pack-ci (planned)

Path: [docs/epics/m15-nuget-pack-ci](epics/m15-nuget-pack-ci/)

- [ ] 01 Charter and pack contract
- [ ] 02 Build/copy/pack scripts (PowerShell)
- [ ] 03 CI win-x64 + nupkg artifact
- [ ] 04 Consumer smoke (Open/Snapshot)
- [ ] 05 Publish path (GitHub Packages) + versioning notes

**Decisions locked in design:** local `artifacts/nuget` → GitHub Packages; win-x64 first; never commit natives; stay on `0.1.x`.

**Handoff:** implement briefs 01→05 in order. Unblocks Mnemon Argos package hosting / monitor integration.

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

- NuGet `Argos` C# wrapper over C ABI (`argos-ffi`) — wrapper only; natives not yet packed
- `argos-bench` harness + `fixtures/small-pnpm`

## Next

1. Implement `m15-nuget-pack-ci` (briefs 01→05)
2. Mnemon integration spike (reference consumer; depends on consumable nupkg)
3. linux-x64 / osx-arm64 pack matrix (after win-x64)

## Blockers

None for planning. Mnemon monitor work is **blocked on** m15 implementation (or temporary ProjectReference escape hatch).
