# Argos execution plan

Live status. Update on every meaningful progress.

## Current

- **Active epic:** none
- **Status:** m16 complete; NuGet CI publishes on main
- **Last update:** 2026-09-02 — `pack-nuget` path-filtered: docs/README-only pushes skip pack/publish; package paths still auto-publish `{csproj}.{run}` on main

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

- [x] Root cause: root-only manifests missed Mnemon nested npm + .NET
- [x] Providers: `npm-nested`, `dotnet-sln`, `dotnet-csproj`; `fallback-root` when still empty
- [x] Planner: package-root `.` when no conventional watch dirs; backends treat `.`
- [x] Fixtures: `nested-npm`, `dotnet-sln`; `small-pnpm` regression
- [x] Mnemon smoke: scopes > 0; watch on `mnemon-web` file edit
- [x] NuGet `Argos` **0.1.2** at `artifacts/nuget/Argos.0.1.2.nupkg`

## Next

1. Push m16 + CI publish change to `main` → Packages gets `0.1.2.<run>` automatically
2. Mnemon PackageReference that published 4-part version (win-x64)
3. linux-x64 / osx-arm64 pack matrix
4. Optional NuGet.org listing after dogfood

## Blockers

None.
