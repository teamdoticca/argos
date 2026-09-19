# Changelog

## Unreleased

- Add MIT license text to source, npm and NuGet distribution.
- Add contribution, security, support, compatibility and release guidance.
- Add pinned toolchain verification, strict cross-platform Rust checks and publication safeguards.
- Document C ABI ownership and synchronization requirements.
- Normalize Rust formatting and conditional backend imports without changing snapshot semantics.
- Upgrade notify to 8.2, removing the unmaintained instant dependency.
- Use locked native builds and shared MSVC toolchain selection; make watch-smoke cleanup safe on timeout.

## 0.1.7

- Discover PHP, Python, Go, Gradle, Maven and operational manifests.
- Share skip-directory and ignore defaults for generated dependency trees.

## 0.1.6

- Watch package-root sources in flat projects, not only their manifests.

## 0.1.5

- Assemble NuGet native assets for win-x64, linux-x64, linux-arm64, osx-arm64 and osx-x64.

## 0.1.4

- Discover document roots and guidance paths.
- Publish the NuGet package publicly using Trusted Publishing.

## Earlier Releases

See [completed epics](docs/roadmap.md) for historical engineering notes. The entries above summarize recorded milestones; they do not imply independently verified GitHub release dates.
