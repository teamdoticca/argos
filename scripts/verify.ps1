[CmdletBinding()]
param([switch] $Docker)

$ErrorActionPreference = 'Stop'
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
if ($Docker) {
    & docker info --format '{{.OSType}}'
    if ($LASTEXITCODE) { throw 'Start Docker Desktop with Linux containers first.' }
    & docker run --rm --mount "type=bind,source=$repoRoot,target=/workspace,readonly" `
        --mount 'type=volume,source=argos-verify-cargo,target=/usr/local/cargo/registry' `
        --mount 'type=volume,source=argos-verify-target,target=/target' `
        -e CARGO_TARGET_DIR=/target -w /workspace rust:1.98.0-bookworm sh -c `
        'cargo fmt --all -- --check && cargo clippy --workspace --all-targets --locked -- -D warnings && cargo test --workspace --locked'
    if ($LASTEXITCODE) { throw 'Docker Linux verification failed.' }
    return
}

. (Join-Path $PSScriptRoot 'enter-msvc.ps1')
Push-Location $repoRoot
try {
    & cargo fmt --all -- --check
    if ($LASTEXITCODE) { throw 'Rust formatting check failed.' }
    & cargo clippy --workspace --all-targets --locked -- -D warnings
    if ($LASTEXITCODE) { throw 'Rust lint failed.' }
    & cargo test --workspace --locked
    if ($LASTEXITCODE) { throw 'Rust tests failed.' }
}
finally { Pop-Location }