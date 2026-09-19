$ErrorActionPreference = 'Stop'
if (-not $IsWindows) { return }
if ($env:VCToolsInstallDir) {
    $desktopLibrary = Join-Path $env:VCToolsInstallDir 'lib/x64/msvcrt.lib'
    $linker = Get-Command link.exe -ErrorAction SilentlyContinue
    if ((Test-Path $desktopLibrary) -and $linker -and
        $linker.Source.StartsWith($env:VCToolsInstallDir, [StringComparison]::OrdinalIgnoreCase) -and
        (($env:LIB -split ';') -contains (Split-Path $desktopLibrary))) {
        Write-Host "MSVC: $env:VCToolsInstallDir (already initialized)"
        return
    }
}
$vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio/Installer/vswhere.exe'
if (-not (Test-Path $vswhere)) { throw 'Install Visual Studio C++ desktop build tools, or use verify.ps1 -Docker for Linux checks.' }
$installations = & $vswhere -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
if ($LASTEXITCODE) { throw 'vswhere failed' }
foreach ($installation in $installations) {
    $devCommand = Join-Path $installation 'Common7/Tools/VsDevCmd.bat'
    $libraries = Get-ChildItem (Join-Path $installation 'VC/Tools/MSVC/*/lib/x64/msvcrt.lib') -ErrorAction SilentlyContinue
    if (-not $libraries -or -not (Test-Path $devCommand)) { continue }
    $environmentLines = & cmd.exe /d /c "`"$devCommand`" -arch=amd64 -host_arch=amd64 >nul && set"
    if ($LASTEXITCODE) { continue }
    foreach ($environmentLine in $environmentLines) {
        if ($environmentLine -match '^([^=]+)=(.*)$') {
            Set-Item -LiteralPath "Env:$($Matches[1])" -Value $Matches[2]
        }
    }
    if (-not (Test-Path (Join-Path $env:VCToolsInstallDir 'lib/x64/msvcrt.lib'))) { continue }
    Write-Host "MSVC: $env:VCToolsInstallDir"
    return
}
throw 'No complete x64 desktop MSVC installation found. Install the C++ desktop workload, or use verify.ps1 -Docker for Linux checks.'