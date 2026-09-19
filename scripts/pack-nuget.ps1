<#
.SYNOPSIS
  Build argos-ffi, stage RID natives, and pack the Argos NuGet package.

.DESCRIPTION
  Stages standard NuGet RID assets under bindings/nuget/Argos/runtimes/<rid>/native/:

    win-x64      → argos_ffi.dll
    osx-arm64    → libargos_ffi.dylib
    osx-x64      → libargos_ffi.dylib
    linux-x64    → libargos_ffi.so
    linux-arm64  → libargos_ffi.so

  Managed wrapper uses DllImport("argos_ffi"). On Unix the CLR loads libargos_ffi.so / .dylib.
  Do not package as argos.dll — collides with managed Argos.dll on case-insensitive filesystems.

.PARAMETER Configuration
  Rust/dotnet configuration. Release maps to cargo --release.

.PARAMETER Version
  Optional package version override (passed as -p:Version=).

.PARAMETER Rids
  One or more RIDs to build/stage. Default: current host RID only.

.PARAMETER AllSupported
  Build/stage every supported RID (only useful where cross-compile works; CI prefers -StageFrom).

.PARAMETER SkipBuild
  Do not run cargo; stage from -StageFrom and/or already-present runtimes files.

.PARAMETER StageFrom
  Directory containing <rid>/native/<filename> trees (CI artifact merge root).

.PARAMETER RequireRids
  RIDs that must exist inside the packed nupkg (fail if missing).
#>
[CmdletBinding()]
param(
    [ValidateSet("Debug", "Release")]
    [string] $Configuration = "Release",

    [string] $Version = "",

    [string[]] $Rids = @(),

    [switch] $AllSupported,

    [switch] $SkipBuild,

    [string] $StageFrom = "",

    [string[]] $RequireRids = @()
)

$ErrorActionPreference = "Stop"

$RidCatalog = [ordered]@{
    "win-x64"      = @{ Triple = "x86_64-pc-windows-msvc";      File = "argos_ffi.dll" }
    "osx-arm64"    = @{ Triple = "aarch64-apple-darwin";         File = "libargos_ffi.dylib" }
    "osx-x64"      = @{ Triple = "x86_64-apple-darwin";          File = "libargos_ffi.dylib" }
    "linux-x64"    = @{ Triple = "x86_64-unknown-linux-gnu";     File = "libargos_ffi.so" }
    "linux-arm64"  = @{ Triple = "aarch64-unknown-linux-gnu";    File = "libargos_ffi.so" }
}

function Get-DefaultHostRid {
    $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString().ToLowerInvariant()
    if ($IsWindows -or ($env:OS -match "Windows")) {
        if ($arch -eq "arm64") { return "win-arm64" } # not in catalog; callers should pass -Rids
        return "win-x64"
    }
    if ($IsMacOS) {
        if ($arch -eq "arm64") { return "osx-arm64" }
        return "osx-x64"
    }
    if ($IsLinux) {
        if ($arch -eq "arm64") { return "linux-arm64" }
        return "linux-x64"
    }
    throw "Unsupported host OS for default RID"
}

function Resolve-CargoArtifact {
    param(
        [string] $RepoRoot,
        [string] $BuildProfile,
        [string] $Triple,
        [string] $FileName
    )
    $candidates = @(
        (Join-Path $RepoRoot "target\$Triple\$BuildProfile\$FileName"),
        (Join-Path $RepoRoot "target/$Triple/$BuildProfile/$FileName"),
        (Join-Path $RepoRoot "target\$BuildProfile\$FileName"),
        (Join-Path $RepoRoot "target/$BuildProfile/$FileName")
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) { return (Resolve-Path $c).Path }
    }
    return $null
}

function Invoke-CargoWithHostToolchain {
    param([string[]] $CargoArgs)

    . (Join-Path $PSScriptRoot "enter-msvc.ps1")
    Write-Host ">> cargo $($CargoArgs -join ' ')"
    & cargo @CargoArgs | Out-Host
    return $LASTEXITCODE
}

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $repoRoot

if ($AllSupported) {
    $Rids = @($RidCatalog.Keys)
}
elseif ($Rids.Count -eq 0) {
    if (-not [string]::IsNullOrWhiteSpace($StageFrom) -and (Test-Path $StageFrom)) {
        $Rids = @(Get-ChildItem $StageFrom -Directory | ForEach-Object { $_.Name } | Where-Object { $RidCatalog.Contains($_) })
        if ($Rids.Count -eq 0) {
            # download-artifact without merge may nest as native-<rid>/<rid>/...
            $Rids = @(
                Get-ChildItem $StageFrom -Directory |
                    ForEach-Object {
                        if ($_.Name -match '^native-(.+)$' -and $RidCatalog.Contains($Matches[1])) {
                            $Matches[1]
                        }
                        elseif ($RidCatalog.Contains($_.Name)) {
                            $_.Name
                        }
                    } |
                    Select-Object -Unique
            )
        }
        if ($Rids.Count -eq 0) {
            throw "No known RID folders under StageFrom: $StageFrom"
        }
    }
    else {
        $hostRid = Get-DefaultHostRid
        if (-not $RidCatalog.Contains($hostRid)) {
            throw "Host RID '$hostRid' is not in the supported NuGet matrix. Pass -Rids explicitly."
        }
        $Rids = @($hostRid)
    }
}

foreach ($rid in $Rids) {
    if (-not $RidCatalog.Contains($rid)) {
        throw "Unsupported RID '$rid'. Supported: $($RidCatalog.Keys -join ', ')"
    }
}

$cargoProfile = if ($Configuration -eq "Release") { "release" } else { "debug" }
$runtimesRoot = Join-Path $repoRoot "bindings\nuget\Argos\runtimes"

foreach ($rid in $Rids) {
    $meta = $RidCatalog[$rid]
    $fileName = $meta.File
    $triple = $meta.Triple
    $nativeDir = Join-Path $runtimesRoot "$rid\native"
    New-Item -ItemType Directory -Force -Path $nativeDir | Out-Null
    $destNative = Join-Path $nativeDir $fileName

    $staged = $false

    if (-not [string]::IsNullOrWhiteSpace($StageFrom)) {
        $fromCandidates = @(
            (Join-Path $StageFrom "$rid\native\$fileName"),
            (Join-Path $StageFrom "native-$rid\native\$fileName"),
            (Join-Path $StageFrom "native-$rid\$rid\native\$fileName"),
            (Join-Path $StageFrom "$rid\$fileName")
        )
        foreach ($from in $fromCandidates) {
            if (Test-Path $from) {
                Copy-Item -Force $from $destNative
                $staged = $true
                Write-Host ">> staged from $from"
                break
            }
        }
    }

    if (-not $staged -and -not $SkipBuild) {
        # Host RID: build without --target so the default MSVC/unix linker env applies.
        # Cross RIDs: require rustup target + --target <triple>.
        $hostRid = $null
        try { $hostRid = Get-DefaultHostRid } catch { $hostRid = $null }
        $cargoArgs = @("build", "-p", "argos-ffi", "--locked")
        if ($rid -ne $hostRid) {
            $cargoArgs += @("--target", $triple)
        }
        if ($Configuration -eq "Release") {
            $cargoArgs += "--release"
        }
        $cargoExit = Invoke-CargoWithHostToolchain -CargoArgs $cargoArgs
        if ($cargoExit -ne 0) {
            throw "cargo build failed for $rid ($triple) with exit code $cargoExit"
        }
        $cargoArtifact = Resolve-CargoArtifact -RepoRoot $repoRoot -BuildProfile $cargoProfile -Triple $triple -FileName $fileName
        if (-not $cargoArtifact) {
            throw "Cargo artifact not found for $rid ($fileName under target/$triple/$cargoProfile or target/$cargoProfile)"
        }
        Copy-Item -Force $cargoArtifact $destNative
        $staged = $true
    }

    if (-not (Test-Path $destNative)) {
        throw "Native missing for ${rid}: expected $destNative (build, StageFrom, or pre-stage under runtimes/)"
    }

    $info = Get-Item $destNative
    if ($info.Length -le 0) {
        throw "Native library is empty: $destNative"
    }
    Write-Host ">> staged $($info.FullName) ($($info.Length) bytes)"
}

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

$nupkgs = Get-ChildItem $outDir -Filter "Argos.*.nupkg" | Sort-Object LastWriteTime -Descending
if ($nupkgs.Count -eq 0) {
    throw "No Argos.*.nupkg found under $outDir"
}

$nupkg = $nupkgs[0].FullName

$verifyRids = @($RequireRids)
if ($verifyRids.Count -eq 0) {
    $verifyRids = @($Rids)
}

$tmpZip = Join-Path ([IO.Path]::GetTempPath()) ("argos-nupkg-" + [guid]::NewGuid().ToString("n") + ".zip")
$tmpExtract = Join-Path ([IO.Path]::GetTempPath()) ("argos-nupkg-" + [guid]::NewGuid().ToString("n"))
try {
    Copy-Item $nupkg $tmpZip
    Expand-Archive -Path $tmpZip -DestinationPath $tmpExtract -Force
    foreach ($requiredFile in @("LICENSE", "PackageReadme.md", "lib/net8.0/Argos.dll")) {
        if (-not (Test-Path (Join-Path $tmpExtract $requiredFile))) {
            throw "Package missing required file: $requiredFile"
        }
    }
    foreach ($rid in $verifyRids) {
        if (-not $RidCatalog.Contains($rid)) {
            throw "RequireRids contains unknown RID '$rid'"
        }
        $fileName = $RidCatalog[$rid].File
        $expectedInside = Join-Path $tmpExtract "runtimes\$rid\native\$fileName"
        if (-not (Test-Path $expectedInside)) {
            $expectedInside = Join-Path $tmpExtract "runtimes/$rid/native/$fileName"
        }
        if (-not (Test-Path $expectedInside)) {
            throw "Packed nupkg is missing runtimes/$rid/native/$fileName"
        }
        Write-Host ">> verified runtimes/$rid/native/$fileName"
    }
}
finally {
    Remove-Item -Force $tmpZip -ErrorAction SilentlyContinue
    Remove-Item -Recurse -Force $tmpExtract -ErrorAction SilentlyContinue
}

Write-Host ">> nupkg $nupkg"
Write-Output $nupkg
