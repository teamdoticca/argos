# Argos execution plan

Live status. Update on every meaningful progress.

## Current

- **Active epic:** [m19-nuget-org-publish](epics/m19-nuget-org-publish/)
- **Status:** in_progress — Trusted Publishing + CI wiring for nuget.org
- **Last update:** 2026-09-03 — Closed m18; m19: pack-nuget dual registry + OIDC publish job

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

## Next

1. Finish **m19** — first nuget.org publish after `NUGET_USER` secret + dispatch `registry=nugetorg`
2. Optional: npm Trusted Publishing (OIDC) and revoke long-lived `NPM_TOKEN`
3. Optional: MCP wrapper epic (`npx` / Cursor tools on `@teamdoticca/argos`)
4. linux-x64 / osx-arm64 NuGet RID matrix

## Blockers

- GitHub Actions secret `NUGET_USER` (nuget.org profile username) required before first public push
