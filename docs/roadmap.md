# Argos roadmap

English only. Update this file on every epic status change.

## Locked core (Product 1 — Workspace Intelligence)

| Epic | Path | Status |
|------|------|--------|
| m00 Foundation | [epics/m00-foundation](epics/m00-foundation/) | in_progress |
| m01 Topology Discovery | [epics/m01-topology-discovery](epics/m01-topology-discovery/) | planned |
| m02 Filesystem Semantics | [epics/m02-filesystem-semantics](epics/m02-filesystem-semantics/) | planned |
| m03 Workspace Model | [epics/m03-workspace-model](epics/m03-workspace-model/) | planned |
| m04 Ignore Engine | [epics/m04-ignore-engine](epics/m04-ignore-engine/) | planned |
| m05 Scope Planner (IP) | [epics/m05-scope-planner](epics/m05-scope-planner/) | planned |
| m06 Diagnostics + Query | [epics/m06-diagnostics-api](epics/m06-diagnostics-api/) | planned |

## Platform / integration (Product 2 + packaging)

| Epic | Path | Status |
|------|------|--------|
| m07 Backend Abstraction | [epics/m07-backend-abstraction](epics/m07-backend-abstraction/) | planned |
| m08 Windows Backend | [epics/m08-windows-backend](epics/m08-windows-backend/) | planned |
| m09 Linux Backend | [epics/m09-linux-backend](epics/m09-linux-backend/) | planned |
| m10 macOS Backend | [epics/m10-macos-backend](epics/m10-macos-backend/) | planned |
| m11 File + Workspace Events | [epics/m11-file-and-workspace-events](epics/m11-file-and-workspace-events/) | planned |
| m12 Platform Recovery | [epics/m12-platform-recovery](epics/m12-platform-recovery/) | planned |
| m13 NuGet Wrapper | [epics/m13-nuget-wrapper](epics/m13-nuget-wrapper/) | planned |
| m14 Benchmark Suite | [epics/m14-benchmark-suite](epics/m14-benchmark-suite/) | planned |

## Done

Completed epics live under [done/](done/).

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
