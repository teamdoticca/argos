$ErrorActionPreference = 'Stop'
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$manifest = Get-Content (Join-Path $repoRoot 'bindings/npm/package.json') -Raw | ConvertFrom-Json
$lock = Get-Content (Join-Path $repoRoot 'bindings/npm/package-lock.json') -Raw | ConvertFrom-Json -AsHashtable
$project = [xml](Get-Content (Join-Path $repoRoot 'bindings/nuget/Argos/Argos.csproj') -Raw)
Push-Location $repoRoot
try {
    $metadataText = & cargo metadata --no-deps --format-version 1 --locked
    if ($LASTEXITCODE) { throw 'Cargo metadata failed' }
    $metadata = $metadataText | ConvertFrom-Json
    foreach ($package in $metadata.packages) {
        if ($package.version -ne $manifest.version) { throw "Version mismatch: $($package.name)" }
    }
    if ($lock.version -ne $manifest.version -or $lock.packages[''].version -ne $manifest.version -or
        $project.Project.PropertyGroup.Version -ne $manifest.version) { throw 'Package versions differ' }
    if ($manifest.author -ne 'Doticca' -or $manifest.license -ne 'MIT' -or
        $project.Project.PropertyGroup.Authors -ne 'Doticca' -or
        $project.Project.PropertyGroup.PackageLicenseExpression -ne 'MIT') { throw 'Package author/license metadata differs' }
    $output = Join-Path $repoRoot 'artifacts/metadata'
    & dotnet pack bindings/nuget/Argos/Argos.csproj --nologo -o $output
    if ($LASTEXITCODE) { throw 'Managed package build failed' }
    $archive = [IO.Compression.ZipFile]::OpenRead((Join-Path $output "Argos.$($manifest.version).nupkg"))
    try {
        foreach ($name in @('LICENSE', 'PackageReadme.md', 'lib/net8.0/Argos.dll')) {
            if (-not $archive.GetEntry($name)) { throw "Missing NuGet entry: $name" }
        }
    }
    finally { $archive.Dispose() }
    Write-Host 'Package versions, ownership and NuGet metadata verified.'
}
finally { Pop-Location }