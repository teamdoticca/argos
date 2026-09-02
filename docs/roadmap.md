# Argos roadmap

English only. Update this file on every epic status change.

## Locked core (Product 1 — Workspace Intelligence)

| Epic | Path | Status |
|------|------|--------|
| m00 Foundation | [done/m00-foundation](done/m00-foundation/) | done |
| m01 Topology Discovery | [done/m01-topology-discovery](done/m01-topology-discovery/) | done |
| m02 Filesystem Semantics | [done/m02-filesystem-semantics](done/m02-filesystem-semantics/) | done |
| m03 Workspace Model | [done/m03-workspace-model](done/m03-workspace-model/) | done |
| m04 Ignore Engine | [done/m04-ignore-engine](done/m04-ignore-engine/) | done |
| m05 Scope Planner (IP) | [done/m05-scope-planner](done/m05-scope-planner/) | done |
| m06 Diagnostics + Query | [done/m06-diagnostics-api](done/m06-diagnostics-api/) | done |

## Platform / integration (Product 2 + packaging)

| Epic | Path | Status |
|------|------|--------|
| m07 Backend Abstraction | [done/m07-backend-abstraction](done/m07-backend-abstraction/) | done |
| m08 Windows Backend | [done/m08-windows-backend](done/m08-windows-backend/) | done |
| m09 Linux Backend | [done/m09-linux-backend](done/m09-linux-backend/) | done |
| m10 macOS Backend | [done/m10-macos-backend](done/m10-macos-backend/) | done |
| m11 File + Workspace Events | [done/m11-file-and-workspace-events](done/m11-file-and-workspace-events/) | done |
| m12 Platform Recovery | [done/m12-platform-recovery](done/m12-platform-recovery/) | done |
| m13 NuGet Wrapper | [done/m13-nuget-wrapper](done/m13-nuget-wrapper/) | done |
| m14 Benchmark Suite | [done/m14-benchmark-suite](done/m14-benchmark-suite/) | done |
| m15 NuGet Pack & CI | [done/m15-nuget-pack-ci](done/m15-nuget-pack-ci/) | done |

## Done

- [m00-foundation](done/m00-foundation/)
- [m01-topology-discovery](done/m01-topology-discovery/)
- [m02-filesystem-semantics](done/m02-filesystem-semantics/)
- [m03-workspace-model](done/m03-workspace-model/)
- [m04-ignore-engine](done/m04-ignore-engine/)
- [m05-scope-planner](done/m05-scope-planner/)
- [m06-diagnostics-api](done/m06-diagnostics-api/)
- [m07-backend-abstraction](done/m07-backend-abstraction/)
- [m08-windows-backend](done/m08-windows-backend/)
- [m09-linux-backend](done/m09-linux-backend/)
- [m10-macos-backend](done/m10-macos-backend/)
- [m11-file-and-workspace-events](done/m11-file-and-workspace-events/)
- [m12-platform-recovery](done/m12-platform-recovery/)
- [m13-nuget-wrapper](done/m13-nuget-wrapper/)
- [m14-benchmark-suite](done/m14-benchmark-suite/)
- [m15-nuget-pack-ci](done/m15-nuget-pack-ci/)

## Deferred

- NPM / napi / TypeScript package implementation (contract exists)
- Shared daemon / IPC
- USN Journal / fanotify
- Network shares optimizations
- WorkspaceDependencyGraph
- Watchman compatibility
- Pluggable strategies (`sparse-root`, `hybrid-poll`)
- Nx / Turborepo / go.work / `.sln`
- Extra `.workspace` fields beyond `imports:`
- Snapshot persistence (`argos snapshot save` / `load`)
- NuGet.org public listing (after GitHub Packages dogfood)
- linux-x64 / osx-arm64 NuGet RID matrix
