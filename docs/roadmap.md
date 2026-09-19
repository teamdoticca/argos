# Argos roadmap

English only. Update this file on every epic status change.

## Locked core (Product 1 — Workspace Intelligence)

| Epic | Path | Status |
| ------ | ------ | -------- |
| m00 Foundation | [done/m00-foundation](done/m00-foundation/) | done |
| m01 Topology Discovery | [done/m01-topology-discovery](done/m01-topology-discovery/) | done |
| m02 Filesystem Semantics | [done/m02-filesystem-semantics](done/m02-filesystem-semantics/) | done |
| m03 Workspace Model | [done/m03-workspace-model](done/m03-workspace-model/) | done |
| m04 Ignore Engine | [done/m04-ignore-engine](done/m04-ignore-engine/) | done |
| m05 Scope Planner (IP) | [done/m05-scope-planner](done/m05-scope-planner/) | done |
| m06 Diagnostics + Query | [done/m06-diagnostics-api](done/m06-diagnostics-api/) | done |

## Platform / integration (Product 2 + packaging)

Public-readiness update 2026-09-18: licensing, contributor guidance, pinned CI, dependency/secret scans and explicit release safeguards implemented. Windows/Linux Rust gates, Windows Node 22/24, Linux Node 24 and Windows NuGet package smokes passed. Remote CI, security settings, registry trust and owner approval remain launch gates; see [release checklist](RELEASING.md).

2026-09-19: current Windows/Linux Rust gates, workflow lint and metadata/license checks passed again. The owner approved a readiness branch and PR for GitHub CI; merge, publication and visibility changes remain separate decisions.

Hosted validation is now green for implementation commit 964b0c8 in [PR #2](https://github.com/teamdoticca/argos/pull/2): Windows/Linux/macOS verification, all npm targets, five NuGet native builds and four RID consumer smokes. Every publish job was skipped. Remaining launch gates are PR approval, ownership/contact confirmation and repository protections; registry trust is required before the next package release.

| Epic | Path | Status |
| ------ | ------ | -------- |
| m07 Backend Abstraction | [done/m07-backend-abstraction](done/m07-backend-abstraction/) | done |
| m08 Windows Backend | [done/m08-windows-backend](done/m08-windows-backend/) | done |
| m09 Linux Backend | [done/m09-linux-backend](done/m09-linux-backend/) | done |
| m10 macOS Backend | [done/m10-macos-backend](done/m10-macos-backend/) | done |
| m11 File + Workspace Events | [done/m11-file-and-workspace-events](done/m11-file-and-workspace-events/) | done |
| m12 Platform Recovery | [done/m12-platform-recovery](done/m12-platform-recovery/) | done |
| m13 NuGet Wrapper | [done/m13-nuget-wrapper](done/m13-nuget-wrapper/) | done |
| m14 Benchmark Suite | [done/m14-benchmark-suite](done/m14-benchmark-suite/) | done |
| m15 NuGet Pack & CI | [done/m15-nuget-pack-ci](done/m15-nuget-pack-ci/) | done |
| m16 Monorepo discovery parity | [done/m16-monorepo-discovery-parity](done/m16-monorepo-discovery-parity/) | done |
| m17 TypeScript napi | [done/m17-typescript-napi](done/m17-typescript-napi/) | done |
| m18 Document root discovery | [done/m18-document-root-discovery](done/m18-document-root-discovery/) | done |
| m19 NuGet.org publish | [done/m19-nuget-org-publish](done/m19-nuget-org-publish/) | done |
| m20 NuGet multi-RID natives | [done/m20-nuget-multi-rid](done/m20-nuget-multi-rid/) | done |
| m21 Planner root source watch | [done/m21-planner-root-source-watch](done/m21-planner-root-source-watch/) | done |
| m22 Polyglot + ops discovery | [done/m22-polyglot-ops-discovery](done/m22-polyglot-ops-discovery/) | done |
| m23 Public repository readiness | [epics/m23-public-readiness](epics/m23-public-readiness/) | in_progress |

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
- [m16-monorepo-discovery-parity](done/m16-monorepo-discovery-parity/)
- [m17-typescript-napi](done/m17-typescript-napi/)
- [m18-document-root-discovery](done/m18-document-root-discovery/)
- [m19-nuget-org-publish](done/m19-nuget-org-publish/)
- [m20-nuget-multi-rid](done/m20-nuget-multi-rid/)
- [m21-planner-root-source-watch](done/m21-planner-root-source-watch/)
- [m22-polyglot-ops-discovery](done/m22-polyglot-ops-discovery/)

## Deferred

- Shared daemon / IPC
- USN Journal / fanotify
- Network shares optimizations
- WorkspaceDependencyGraph
- Watchman compatibility
- Pluggable strategies (`sparse-root`, `hybrid-poll`)
- Nx / Turborepo
- Extra `.workspace` fields beyond `imports:`
- Snapshot persistence (`argos snapshot save` / `load`)
- linux-musl NuGet RIDs
- linux-arm64 / darwin-x64 / musl napi triples
- Author NuGet package signing (code-signing certificates)
