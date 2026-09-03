# Argos execution plan

Live status. Update on every meaningful progress.

## Current

- **Active epic:** none
- **Status:** m22 closed; awaiting next epic
- **Last update:** 2026-09-03 — Closed m22; public **Argos 0.1.7** (polyglot + ops discovery)

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
- [x] Public **Argos 0.1.4** — https://www.nuget.org/packages/Argos/0.1.4

### m20-nuget-multi-rid (done)

Path: [docs/done/m20-nuget-multi-rid](done/m20-nuget-multi-rid/)

- [x] One multi-RID nupkg: win-x64, osx-arm64, osx-x64, linux-x64, linux-arm64 (glibc)
- [x] CI matrix + assemble + smoke (osx-x64 pack-verified)
- [x] Public **Argos 0.1.5** — https://www.nuget.org/packages/Argos/0.1.5
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

## Next

1. Mnemon PackageReference bump to `0.1.7` (separate repo); optional live Intalepoint smoke
2. Optional: drop Windows-only RID default in Mnemon.Api
3. Optional: npm Trusted Publishing (OIDC) and revoke long-lived `NPM_TOKEN`
4. Optional: MCP wrapper epic (`npx` / Cursor tools on `@teamdoticca/argos`)

## Blockers

None.
