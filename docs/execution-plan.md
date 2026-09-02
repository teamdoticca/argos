# Argos execution plan

Live status. Update on every meaningful progress.

## Current

- **Active epic:** [m17-typescript-napi](epics/m17-typescript-napi/)
- **Status:** in_progress
- **Last update:** 2026-09-02 — m17: `argos-napi` + `@doticca/argos`; win smoke/watch green; `pack-npm.yml` matrix win/linux/darwin-arm64

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

1. Finish m17: `argos-napi`, npm package, multi-OS CI, smoke, README Node section
2. Mnemon PackageReference dogfood (NuGet)
3. linux-x64 / osx-arm64 NuGet RID matrix
4. Optional npmjs / NuGet.org listing after dogfood

## Blockers

None.
