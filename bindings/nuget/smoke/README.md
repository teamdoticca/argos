# Argos NuGet smoke

PackageReference consumer that proves Open → Snapshot works against a packed nupkg.

## Prerequisites

1. Pack locally: `pwsh ./scripts/pack-nuget.ps1`
2. .NET 8 SDK

## Run

```powershell
pwsh ./scripts/pack-nuget.ps1
dotnet run --project bindings/nuget/smoke/Argos.Smoke.csproj -- fixtures/small-pnpm
```

Or pass an explicit package version:

```powershell
dotnet run --project bindings/nuget/smoke/Argos.Smoke.csproj -p:ArgosPackageVersion=0.1.0 -- fixtures/small-pnpm
```

## Failure UX

If `argos_ffi.dll` is missing for the RID, Open fails with `DllNotFoundException` / native load error. Re-run `pack-nuget.ps1` and restore the smoke project.

Watch is intentionally **not** required for smoke (Product 1 only).
