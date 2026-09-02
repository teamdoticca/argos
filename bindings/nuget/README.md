# Argos NuGet package

.NET wrapper over the Argos C ABI (`argos_ffi` cdylib).

## Usage

```csharp
var workspace = await ArgosWorkspace.OpenAsync(repoPath);
var snapshot = workspace.CurrentSnapshot;
await foreach (var change in workspace.WatchAsync())
{
    var affected = workspace.GetAffectedScopes(change);
}
```

## Pack (local)

```powershell
pwsh ./scripts/pack-nuget.ps1 -Rid win-x64
```

Produces `artifacts/nuget/Argos.<version>.nupkg` with:

- `lib/net8.0/Argos.dll`
- `runtimes/win-x64/native/argos_ffi.dll`

Cargo builds `argos_ffi.dll`; the pack script stages it under that same name for `DllImport("argos_ffi")` (avoids colliding with managed `Argos.dll` on Windows).

Natives are **never** committed. Smoke: `bindings/nuget/smoke/`.

## Publish (GitHub Packages)

CI workflow: `.github/workflows/pack-nuget.yml`

| Trigger | Pack + smoke | Publish to GitHub Packages |
|---------|--------------|----------------------------|
| Push to `master` / PR | yes | no |
| Tag `v*` (e.g. `v0.1.1`) | yes | yes |
| Actions → **Run workflow** (`workflow_dispatch`, publish=true) | yes | yes |

After publish, the package appears under the org packages page for `teamdoticca`.

### Consumer feed

```text
https://nuget.pkg.github.com/teamdoticca/index.json
```

Needs a GitHub PAT with `read:packages` (and org SSO authorized if required).

## RIDs

| RID | Status |
|-----|--------|
| `win-x64` | Current pack script + CI |
| `linux-x64` | Deferred |
| `osx-arm64` | Deferred |

## Consume (Mnemon / dogfood)

1. Pack or download CI artifact `argos-nuget-win-x64`
2. Add a NuGet source to `artifacts/nuget` (or GitHub Packages)
3. `<PackageReference Include="Argos" Version="0.1.x" />`
4. Host RID `win-x64`

### Rollback

ProjectReference `bindings/nuget/Argos/Argos.csproj` and place `argos_ffi.dll` next to the host output.

## Versioning

Stay on `0.1.x` for packaging/native rebuilds. Minor bumps when public managed/C ABI changes.

Watching is always optional — Product 1 (Workspace Intelligence) works without `WatchAsync`.
