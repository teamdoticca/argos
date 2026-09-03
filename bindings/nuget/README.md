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

Publish or run the host with a supported RID so native assets resolve:

```bash
dotnet publish -r osx-arm64
dotnet publish -r linux-x64
dotnet publish -r win-x64
```

## Pack (local)

Host RID only (default):

```powershell
pwsh ./scripts/pack-nuget.ps1
# or: pwsh ./scripts/pack-nuget.ps1 -Rids win-x64
```

Assemble a multi-RID package from prebuilt natives (CI):

```powershell
pwsh ./scripts/pack-nuget.ps1 -SkipBuild -StageFrom artifacts/natives `
  -RequireRids win-x64,osx-arm64,linux-x64,osx-x64,linux-arm64
```

Produces `artifacts/nuget/Argos.<version>.nupkg` with `lib/net8.0/Argos.dll` plus RID natives below.

Managed wrapper uses `DllImport("argos_ffi")`. On Unix the CLR loads `libargos_ffi.so` / `libargos_ffi.dylib`. On Windows the packaged name must **not** be `argos.dll` (collides with managed `Argos.dll`).

Natives are **never** committed. Smoke: `bindings/nuget/smoke/` (`--watch` for watch events).

## Supported RIDs

| RID | Native filename | Notes |
|-----|-----------------|-------|
| `win-x64` | `argos_ffi.dll` | Required |
| `osx-arm64` | `libargos_ffi.dylib` | Apple Silicon |
| `osx-x64` | `libargos_ffi.dylib` | Intel Mac |
| `linux-x64` | `libargos_ffi.so` | **glibc** (`x86_64-unknown-linux-gnu`) |
| `linux-arm64` | `libargos_ffi.so` | **glibc** (`aarch64-unknown-linux-gnu`) |

**Not in this package:** `linux-musl-*` / Alpine (deferred), Windows ARM (optional later).

## Publish

CI workflow: `.github/workflows/pack-nuget.yml`

Matrix builds each RID native → one multi-RID nupkg → smoke per RID → publish.

| Trigger | Pack + smoke | GitHub Packages | nuget.org |
|---------|--------------|-----------------|-----------|
| Push to `master` / PR | yes | yes on main (`{base}.{run}`) | no |
| Tag `v*` | yes | yes (tag version) | no |
| `workflow_dispatch` `registry=github` | yes | yes | no |
| `workflow_dispatch` `registry=nugetorg` | yes | no | yes (exact `{base}`, OIDC) |
| `workflow_dispatch` `registry=both` | yes | yes | yes |

nuget.org uses **Trusted Publishing** (`NuGet/login@v1`). See [nugetorg-prep.md](../../docs/done/m19-nuget-org-publish/nugetorg-prep.md).

### Dogfood feed (optional)

```text
https://nuget.pkg.github.com/teamdoticca/index.json
```

Needs a GitHub PAT with `read:packages`.

## Versioning

Stay on `0.1.x` for packaging/native rebuilds. Minor bumps when public managed/C ABI changes.

Watching is always optional — Product 1 (Workspace Intelligence) works without `WatchAsync`.
