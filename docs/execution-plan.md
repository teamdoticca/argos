# Argos execution plan

Live status. Update on every meaningful progress.

## Current

- **Active epic:** None.
- **Status:** m24 completed; all six initial Dependabot proposals consolidated and verified without changing public APIs or package versions.
- **Owner approval (2026-09-19):** Policies, monitored mailbox and distribution rights confirmed; completion of public launch authorized after readiness checks passed.
- **Last update:** 2026-09-19 - PR #10 implementation head a2cf551 passed all hosted verify/npm/NuGet workflows. m24 archived with evidence. Owner approved green-only merge and subsequent closure of superseded proposals; final documentation-head checks remain a merge gate. No publication or credential changes.

## Completed

### m00-foundation (done)

- Local repo, GitHub, English docs, agents/rules/skills

### m01–m06 Workspace Intelligence (done)

- Topology discovery through diagnostics/query APIs

### m07–m12 Change Tracking platform (done)

- Backend abstraction, OS backends, events, recovery

### m13–m14 Packaging & benchmarks (done)

- NuGet C# wrapper; benchmarks + fixtures

### m15-nuget-pack-ci (done)

Path: [docs/done/m15-nuget-pack-ci](done/m15-nuget-pack-ci/)

- Pack/CI for win-x64; native `argos_ffi.dll`; GitHub Packages dogfood

### m16-monorepo-discovery-parity (done)

Path: [docs/done/m16-monorepo-discovery-parity](done/m16-monorepo-discovery-parity/)

- Nested npm + .NET discovery; Mnemon scopes; NuGet `0.1.2`

### m17-typescript-napi (done)

Path: [docs/done/m17-typescript-napi](done/m17-typescript-napi/)

- [x] `argos-napi` + `@teamdoticca/argos`
- [x] win / linux-x64 / darwin-arm64 CI + smoke/watch
- [x] GitHub Packages + **npmjs.org** public `0.1.2`

### m18-document-root-discovery (done)

Path: [docs/done/m18-document-root-discovery](done/m18-document-root-discovery/)

- [x] Scored `document-root` + `guidance-path` discovery
- [x] Shallow docs root preference; release line `0.1.4`

### m19-nuget-org-publish (done)

Path: [docs/done/m19-nuget-org-publish](done/m19-nuget-org-publish/)

- [x] Trusted Publishing OIDC + dual-registry `pack-nuget`
- [x] Gallery metadata + README primary install via nuget.org
- [x] Public **Argos 0.1.4** — <https://www.nuget.org/packages/Argos/0.1.4>

### m20-nuget-multi-rid (done)

Path: [docs/done/m20-nuget-multi-rid](done/m20-nuget-multi-rid/)

- [x] One multi-RID nupkg: win-x64, osx-arm64, osx-x64, linux-x64, linux-arm64 (glibc)
- [x] CI matrix + assemble + smoke (osx-x64 pack-verified)
- [x] Public **Argos 0.1.5** — <https://www.nuget.org/packages/Argos/0.1.5>
- [x] npm line **`@teamdoticca/argos@0.1.5`** on npmjs; GH Packages dogfood `--tag latest`

### m21-planner-root-source-watch (done)

Path: [docs/done/m21-planner-root-source-watch](done/m21-planner-root-source-watch/)

- [x] Flat package roots plan `watch: ["."]` (Tray / Api / Electron-style)
- [x] Planner unit tests
- [x] Public **Argos 0.1.6** — nuget.org + GitHub Packages; npm `0.1.6`

### m22-polyglot-ops-discovery (done)

Path: [docs/done/m22-polyglot-ops-discovery](done/m22-polyglot-ops-discovery/)

- [x] PHP / Python / Go / Android-Gradle / Maven / ops (Dockerfile, Compose, Bicep, azure.yaml)
- [x] Shared skip-dirs + ignore hard-defaults (`vendor`, `.venv`, …)
- [x] Fixture `mixed-polyglot-ops`; `cargo test -p argos-core` green
- [x] Public **Argos 0.1.7** — nuget.org + GitHub Packages; npm `0.1.7`

### m23-public-readiness (done)

Path: [docs/done/m23-public-readiness](done/m23-public-readiness/)

- [x] Licensing, community policies, cross-platform CI, package checks and dependency/history scans.
- [x] [PR #2](https://github.com/teamdoticca/argos/pull/2) merged after verify/npm/NuGet passed on final readiness head b4ed22c.
- [x] Public visibility and anonymous access verified; master/version-tag protections and public security controls enabled.
- [x] Package publication and credential rotation left to a separately authorized release.

### m24-dependency-upgrades (done)

Path: [docs/done/m24-dependency-upgrades](done/m24-dependency-upgrades/)

- [x] TOML 1.1 document parsing, coordinated napi 3/CLI migration and uuid/napi-build patches.
- [x] Clean npm lockfile, expanded Node API smokes and grouped Cargo napi updates.
- [x] Implementation head a2cf551 passed verify 35421342658, npm 35421342774 and NuGet 35421342777; publication skipped.

## Next

1. Merge PR #10 only after final-head checks pass and close superseded PRs #3 through #8. Continue maintaining public issues, vulnerability reports and dependency alerts.
2. Before a separately authorized package release, verify protected environments and registry trust, choose a new version, and validate OIDC before revoking the old npm token. Follow the [release checklist](RELEASING.md).
3. Later: Mnemon integration updates and an optional MCP wrapper epic, in separately scoped work. No epic beyond m24 is started.

## Blockers

No implementation blockers remain. All m24 hosted matrices and audits passed; final PR-head checks remain mandatory before merge. Registry trust and publication-environment verification remain prerequisites for the next package release, not unfinished repository launch work.
