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

### GitHub Packages

On `workflow_dispatch` or tags `v*`, CI can push to `https://nuget.pkg.github.com/teamdoticca/index.json` (see `.github/workflows/pack-nuget.yml`).

### Rollback

ProjectReference `bindings/nuget/Argos/Argos.csproj` and place `argos.dll` next to the host output.

## Versioning

Stay on `0.1.x` for packaging/native rebuilds. Minor bumps when public managed/C ABI changes.

Watching is always optional — Product 1 (Workspace Intelligence) works without `WatchAsync`.
