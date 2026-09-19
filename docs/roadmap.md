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

Public launch completed on 2026-09-19 with owner approval. [PR #2](https://github.com/teamdoticca/argos/pull/2) merged after final readiness head b4ed22c passed Windows/Linux/macOS verification, npm native targets and NuGet builds/consumer smokes. Licensing, community policies, pinned CI and dependency/history scans are in place. Intel Mac remains pack-verified only.

Argos is public; anonymous clone and README access succeeded. Master requires PRs, six strict CI checks, resolved conversations and admin enforcement, with no force pushes/deletion and no second-reviewer requirement. Version tags are protected against updates/deletion. Secret scanning, push protection, Dependabot security updates and private vulnerability reporting are enabled. The separately authorized 0.1.8 release completed protected OIDC publication and credential retirement; see the [release checklist](RELEASING.md).

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
| m23 Public repository readiness | [done/m23-public-readiness](done/m23-public-readiness/) | done |
| m24 Dependency upgrade compatibility | [done/m24-dependency-upgrades](done/m24-dependency-upgrades/) | done |
| m25 Package release 0.1.8 | [done/m25-package-release](done/m25-package-release/) | done |

Dependency maintenance completed 2026-09-19: [PR #10](https://github.com/teamdoticca/argos/pull/10) consolidated TOML document parsing, coordinated napi 3/CLI migration, uuid/napi-build patches and Cargo napi grouping. Final PR-head and merged-master workflows passed; superseded proposals #3 through #8 are closed. Local Windows/Linux strict Rust gates and 38 tests each also passed. Evidence is archived with m24; the changes shipped in the separately authorized m25 release.

Packaging maintenance: the managed Argos NuGet wrapper now has a stable public strong-name identity and targets .NET 8, .NET 9, and .NET 10 for C# consumers. This does not change the WorkspaceSnapshot contract, native artifacts, or NuGet publication policy.

The next package release is being prepared as 0.1.9; publication remains pending the standard release gates and owner-authorized tag workflow.

## Done

Release 0.1.8 completed on 2026-09-19 with owner authorization. PR #11 merged as 96bb168 after all checks passed. Both dry runs, protected OIDC publications, fresh public consumers and registry signature/provenance checks passed. The protected v0.1.8 tag and [GitHub release](https://github.com/teamdoticca/argos/releases/tag/v0.1.8) identify the same published SHA. Actual registry artifacts and hashes are attached; Intel Mac remains pack-verified only. No epic is active.

The npmjs/nugetorg environments require maintainer approval and restrict deployment to master; registry trust is verified. The old npm token was revoked after OIDC succeeded and the unused NPM_TOKEN secret was removed.

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
- [m23-public-readiness](done/m23-public-readiness/)
- [m24-dependency-upgrades](done/m24-dependency-upgrades/)
- [m25-package-release](done/m25-package-release/)

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
-
