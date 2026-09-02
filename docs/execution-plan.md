# Argos execution plan

Live status. Update on every meaningful progress.

## Current

- **Active epic:** [m17-typescript-napi](epics/m17-typescript-napi/)
- **Status:** in_progress
- **Last update:** 2026-09-02 — npmjs prep wired (checklist, `NPM_TOKEN` publish path, dual-registry workflow)

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

## Next

1. Verify m17 `pack-npm` CI green (win/linux/darwin-arm64)
2. Human npmjs prep ([npmjs-prep.md](epics/m17-typescript-napi/npmjs-prep.md)): org/scope `@teamdoticca`, secret `NPM_TOKEN`, then workflow_dispatch `registry=npmjs`
3. Switch README Node install to plain `npm install @teamdoticca/argos` after first npmjs publish
4. Optional: MCP wrapper epic once public install works
5. Mnemon PackageReference dogfood (NuGet)
6. linux-x64 / osx-arm64 NuGet RID matrix

## Blockers

None.
