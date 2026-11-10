# Changelog

## Unreleased

## 0.1.11

- Ignore non-mutating filesystem access notifications (open, read, close-after-read) before the bounded watch queue so read-only scans cannot fake `fileModified` or overflow into `scopeRescan`.
- Preserve close-after-write (`Access(Close(Write))` / ambiguous `Close(Any)`) as `fileModified`; keep create/modify/delete/rename/topology and genuine overflow recovery.
- Add Linux access-noise integration tests and Bookworm Node `followConfined` idle-scan validation via `scripts/validate-access-noise-bookworm.sh`.

## 0.1.10

- Add opt-in filesystem-only open mode (`mode: "filesystem"`) for explicitly registered directory roots.
- Add consumer include/exclude path filters, coverage diagnostics and symlink policies (`nofollow`, `followConfined`, `follow`).
- Surface rename hints (`fromPath`/`toPath`) and recovery reasons on file events; bound watch event buffering with overflow → `scopeRescan`.
- Preserve workspace-mode defaults and topology discovery when filesystem mode is not selected.
- Document Debian Bookworm + Node 24 packaged-addon validation via `scripts/validate-filesystem-bookworm.sh` (Bookworm-built native).
- Replace deprecated `serde_yaml` with maintained `serde_norway` to clear `RUSTSEC-2024-0449` (`unsafe-libyaml` unmaintained).

## 0.1.9

- Add managed NuGet targets for .NET 8, .NET 9, and .NET 10.
- Add a stable public strong-name identity to the managed Argos wrapper.
- Expand RID consumer smoke coverage across all supported .NET target frameworks.

## 0.1.8

- Add MIT license text to source, npm and NuGet distribution.
- Add contribution, security, support, compatibility and release guidance.
- Add pinned toolchain verification, strict cross-platform Rust checks and publication safeguards.
- Document C ABI ownership and synchronization requirements.
- Normalize Rust formatting and conditional backend imports without changing snapshot semantics.
- Upgrade notify to 8.2, removing the unmaintained instant dependency.
- Use locked native builds and shared MSVC toolchain selection; make watch-smoke cleanup safe on timeout.
- Migrate to toml 1.1 document parsing and coordinated napi 3 bindings/CLI, preserving the public Node API and locked native builds.
- Update uuid and napi-build patches, group coupled Cargo napi upgrades, and expand Node API compatibility smoke checks.

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
