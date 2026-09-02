<#
.SYNOPSIS
  Build argos-ffi, stage RID natives, and pack the Argos NuGet package.

.DESCRIPTION
  Cargo artifact (Windows): target/<profile>/argos_ffi.dll
  Packaged native name:     runtimes/<rid>/native/argos_ffi.dll  (DllImport "argos_ffi")
  Note: do not use argos.dll — collides with managed Argos.dll on Windows.
.PARAMETER Configuration
  Rust/dotnet configuration. Release maps to cargo --release.

.PARAMETER Version
  Optional package version override (passed as -p:Version=).

.PARAMETER Rid
  Runtime identifier. First supported: win-x64.

.PARAMETER SkipBuild
  Skip cargo build (use existing artifact).
#>
[CmdletBinding()]
param(
    [ValidateSet("Debug", "Release")]
    [string] $Configuration = "Release",

    [string] $Version = "",

    [ValidateSet("win-x64")]
    [string] $Rid = "win-x64",

    [switch] $SkipBuild
)

$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $repoRoot

$cargoProfile = if ($Configuration -eq "Release") { "release" } else { "debug" }
$cargoArgs = @("build", "-p", "argos-ffi")
if ($Configuration -eq "Release") {
    $cargoArgs += "--release"
}

if (-not $SkipBuild) {
    Write-Host ">> cargo $($cargoArgs -join ' ')"
    & cargo @cargoArgs
    if ($LASTEXITCODE -ne 0) {
        throw "cargo build failed with exit code $LASTEXITCODE"
    }
}

# Cargo renames crate argos-ffi → argos_ffi.dll on Windows.
# Packaged native keeps that name so it does not collide with managed Argos.dll on case-insensitive FS.
$cargoArtifact = Join-Path $repoRoot "target\$cargoProfile\argos_ffi.dll"
if (-not (Test-Path $cargoArtifact)) {
    throw "Cargo artifact not found: $cargoArtifact"
}

$nativeDir = Join-Path $repoRoot "bindings\nuget\Argos\runtimes\$Rid\native"
New-Item -ItemType Directory -Force -Path $nativeDir | Out-Null
$destNative = Join-Path $nativeDir "argos_ffi.dll"
Copy-Item -Force $cargoArtifact $destNative

$info = Get-Item $destNative
if ($info.Length -le 0) {
    throw "Native library is empty: $destNative"
}
Write-Host ">> staged $($info.FullName) ($($info.Length) bytes)"

$outDir = Join-Path $repoRoot "artifacts\nuget"
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

$csproj = Join-Path $repoRoot "bindings\nuget\Argos\Argos.csproj"
$packArgs = @(
    "pack", $csproj,
    "-c", $Configuration,
    "-o", $outDir,
    "--nologo"
)
if (-not [string]::IsNullOrWhiteSpace($Version)) {
    $packArgs += "-p:Version=$Version"
}

Write-Host ">> dotnet $($packArgs -join ' ')"
& dotnet @packArgs
if ($LASTEXITCODE -ne 0) {
    throw "dotnet pack failed with exit code $LASTEXITCODE"
}

# Fail if declared RID native was not included (sanity: staged file must still exist).
if (-not (Test-Path $destNative)) {
    throw "RID native missing after pack staging: $destNative"
}

$nupkgs = Get-ChildItem $outDir -Filter "Argos.*.nupkg" | Sort-Object LastWriteTime -Descending
if ($nupkgs.Count -eq 0) {
    throw "No Argos.*.nupkg found under $outDir"
}

$nupkg = $nupkgs[0].FullName

# Verify nupkg contains win-x64 native (requires Expand-Archive on zip copy).
$tmpZip = Join-Path $env:TEMP ("argos-nupkg-" + [guid]::NewGuid().ToString("n") + ".zip")
$tmpExtract = Join-Path $env:TEMP ("argos-nupkg-" + [guid]::NewGuid().ToString("n"))
try {
    Copy-Item $nupkg $tmpZip
    Expand-Archive -Path $tmpZip -DestinationPath $tmpExtract -Force
    $expectedInside = Join-Path $tmpExtract "runtimes\$Rid\native\argos_ffi.dll"
    if (-not (Test-Path $expectedInside)) {
        throw "Packed nupkg is missing runtimes/$Rid/native/argos_ffi.dll"
    }
}
finally {
    Remove-Item -Force $tmpZip -ErrorAction SilentlyContinue
    Remove-Item -Recurse -Force $tmpExtract -ErrorAction SilentlyContinue
}

Write-Host ">> nupkg $nupkg"
Write-Output $nupkg
