# 03 — CI win-x64

**Status:** planned  
**Epic:** [m15-nuget-pack-ci](./README.md)  
**Depends on:** [02-build-copy-pack-scripts](./02-build-copy-pack-scripts.md)

## Outcome

Describe the CI job shape that packs Argos for win-x64 and publishes build artifacts (not necessarily NuGet.org).

## Job shape (GitHub Actions — to implement later)

Suggested workflow name: `pack-nuget.yml` (or a job inside existing CI once CI exists).

```text
on: push / pull_request / workflow_dispatch
runs-on: windows-latest
steps:
  1. Checkout
  2. Install Rust stable (MSVC)
  3. Install .NET 8 SDK
  4. cargo test -p argos-core (gate)
  5. Run scripts/pack-nuget.ps1 -Rid win-x64
  6. Upload-Artifact: artifacts/nuget/*.nupkg
  7. (Optional later) Push to GitHub Packages on tag / main
```

## Artifacts

- Name: `argos-nuget-win-x64`
- Contents: `Argos.<version>.nupkg`
- Retention: short for PR builds; longer for release tags

## Failure conditions

| Condition | Result |
|-----------|--------|
| Rust / .NET install fails | Job fail |
| `argos-core` tests fail | Job fail (no pack) |
| Pack script non-zero | Job fail |
| Nupkg missing after pack | Job fail |
| Empty runtimes in nupkg | Job fail (inspect via `dotnet` / unzip check optional) |

## Multi-RID timeline

- **Now (this epic):** windows-latest only
- **Later:** matrix `windows-latest` / `ubuntu-latest` / `macos-14` producing separate artifacts or a multi-RID nupkg assembled carefully (prefer one nupkg with multiple `runtimes/` entries once all builds succeed)

## Exit criteria (impl)

- [ ] PR can download a win-x64 nupkg artifact
- [ ] Main/tag policy for Packages push documented in brief 05

## Non-goals

linux/macos CI in the first PR unless cost-free.
