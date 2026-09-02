# 05 — Publish and versioning

**Status:** done  
**Epic:** [m15-nuget-pack-ci](./README.md)  
**Depends on:** [03-ci-win-x64](./03-ci-win-x64.md), [04-consumer-smoke](./04-consumer-smoke.md)

## Outcome

Lock feed choice, semver policy, and exact Mnemon consume steps.

## Feed choice

| Stage | Feed | When |
|-------|------|------|
| Dev / PR | `artifacts/nuget` local folder | Always |
| Mnemon dogfood | **GitHub Packages** for `teamdoticca/argos` | After smoke green on main |
| Public | NuGet.org | Deferred (not this epic’s exit) |

### Why not NuGet.org first

Private/early API surface; Mnemon is the reference consumer; Packages auth already fits Doticca GitHub org workflow.

## Versioning

- Package base version lives in `Argos.csproj` (e.g. `0.1.2`)
- **Patch** (`0.1.x`): packaging, native rebuild, discovery fixes, CI
- **Minor** (`0.2.0`): public managed/C ABI change consumers must adopt
- **CI on `main`/`master` push (package-relevant paths only):** pack + publish as `{csproj Version}.{GITHUB_RUN_NUMBER}` (4-part, unique; NuGet `+metadata` is not unique)
- **Path filter:** workflow runs only when `crates/**`, `bindings/nuget/**`, `scripts/pack-nuget.ps1`, workflow file, workspace `Cargo.toml`/`Cargo.lock`, or `fixtures/**` change — docs/README-only pushes do **not** pack or publish
- **PR:** pack only as `{base}-pr.{run_number}` (no Packages publish), same path filter
- **Tag `v*`:** pack + publish exact tag version (optional stable pin)
- **workflow_dispatch:** always available; publish only when ref is `main`/`master`

`WorkspaceSnapshot.schemaVersion` is separate; include both in release notes.

## Mnemon consume steps (exact)

1. Obtain `Argos.0.1.x.nupkg` (CI artifact or Packages)
2. Add NuGet source (example):

```xml
<!-- Mnemon NuGet.config fragment -->
<add key="argos-local" value="C:\path\to\argos\artifacts\nuget" />
<!-- or GitHub Packages URL for teamdoticca -->
```

3. PackageReference:

```xml
<PackageReference Include="Argos" Version="0.1.x" />
```

4. Ensure host publishes for `win-x64` (RID-aware publish if single-file)
5. Call `ArgosWorkspace.OpenAsync(repoRoot)` in monitor startup when `watchWorkingTree` enabled
6. Keep ProjectReference escape hatch until Packages feed is trusted

## Rollback

- Revert PackageReference → ProjectReference to `bindings/nuget/Argos/Argos.csproj`
- Copy `argos.dll` next to host output manually
- Disable Argos-backed monitor feature flag in Mnemon

## Exit criteria (impl)

- [ ] Documented publish path for GitHub Packages (secrets, workflow_dispatch or tag)
- [ ] Version bump checklist in epic or scripts README
- [ ] Mnemon can install without cloning Argos source (happy path)

## Non-goals

NuGet.org listing; signing; multi-feed mirroring.
