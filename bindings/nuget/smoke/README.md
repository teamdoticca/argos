# Argos NuGet smoke

PackageReference consumer that proves Open → Snapshot (and optional Watch) against a packed nupkg.

## Prerequisites

1. Pack locally: `pwsh ./scripts/pack-nuget.ps1`
2. .NET 8 SDK

## Run

```powershell
pwsh ./scripts/pack-nuget.ps1
dotnet run --project bindings/nuget/smoke/Argos.Smoke.csproj -p:RuntimeIdentifier=win-x64 -- fixtures/small-pnpm
dotnet run --project bindings/nuget/smoke/Argos.Smoke.csproj -p:RuntimeIdentifier=win-x64 -- --watch
```

Or pass an explicit package version:

```powershell
dotnet run --project bindings/nuget/smoke/Argos.Smoke.csproj `
  -p:ArgosPackageVersion=0.1.5 -p:RuntimeIdentifier=win-x64 -- fixtures/small-pnpm
```

## Failure UX

If the RID native is missing, Open fails with `DllNotFoundException` / native load error. Re-run `pack-nuget.ps1` (or CI multi-RID pack) and restore the smoke project.
