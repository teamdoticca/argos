# Argos execution plan

Live status. Update on every meaningful progress.

## Current

- **Active epic:** none
- **Status:** m15 complete
- **Last update:** 2026-09-02 — NuGet pack/CI shipped (win-x64); Mnemon can PackageReference

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

- [x] Charter + gitignore for `artifacts/`
- [x] `scripts/pack-nuget.ps1` (cargo → stage `argos_ffi.dll` → pack)
- [x] CI `.github/workflows/pack-nuget.yml` (test, pack, artifact, smoke; optional Packages push)
- [x] Smoke PackageReference consumer (`bindings/nuget/smoke`)
- [x] Publish/versioning docs (`0.1.1`; GitHub Packages on tag/`workflow_dispatch`)

**Note:** Native library is `argos_ffi.dll` (not `argos.dll`) to avoid colliding with managed `Argos.dll` on Windows.

## Next

1. Mnemon integration spike (PackageReference `Argos` 0.1.x, win-x64)
2. linux-x64 / osx-arm64 pack matrix
3. Optional NuGet.org listing after dogfood

## Blockers

None.
