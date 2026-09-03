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

## Install (public)

```bash
dotnet add package Argos
```

Gallery: https://www.nuget.org/packages/Argos

## Pack (local)

```powershell
pwsh ./scripts/pack-nuget.ps1 -Rid win-x64
```

Produces `artifacts/nuget/Argos.<version>.nupkg` with:

- `lib/net8.0/Argos.dll`
- `runtimes/win-x64/native/argos_ffi.dll`

Cargo builds `argos_ffi.dll`; the pack script stages it under that same name for `DllImport("argos_ffi")` (avoids colliding with managed `Argos.dll` on Windows).

Natives are **never** committed. Smoke: `bindings/nuget/smoke/`.

## Publish

CI workflow: `.github/workflows/pack-nuget.yml`

| Trigger | Pack + smoke | GitHub Packages | nuget.org |
|---------|--------------|-----------------|-----------|
| Push to `master` / PR | yes | yes on main (`{base}.{run}`) | no |
| Tag `v*` | yes | yes (tag version) | no |
| `workflow_dispatch` `registry=github` | yes | yes | no |
| `workflow_dispatch` `registry=nugetorg` | yes | no | yes (exact `{base}`, OIDC) |
| `workflow_dispatch` `registry=both` | yes | yes | yes |

nuget.org uses **Trusted Publishing** (`NuGet/login@v1`). See [nugetorg-prep.md](../../docs/epics/m19-nuget-org-publish/nugetorg-prep.md).

### Dogfood feed (optional)

```text
https://nuget.pkg.github.com/teamdoticca/index.json
```

Needs a GitHub PAT with `read:packages`.

## RIDs

| RID | Status |
|-----|--------|
| `win-x64` | Current pack script + CI |
| `linux-x64` | Deferred |
| `osx-arm64` | Deferred |

## Versioning

Stay on `0.1.x` for packaging/native rebuilds. Minor bumps when public managed/C ABI changes.

Watching is always optional — Product 1 (Workspace Intelligence) works without `WatchAsync`.
