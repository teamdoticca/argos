# Status

done

## Implemented

- MIT license and Doticca attribution; verified license/readme inclusion in npm and NuGet archives.
- Contributor, conduct, security, support, compatibility and release guidance; issue and PR templates; Dependabot configuration.
- Committed-lockfile policy, pinned Rust toolchain, strict cross-platform format/lint/test workflow, version/archive checks, dependency audits and full-history secret scanning.
- Immutable action revisions and scanner images, credential-free checkouts, read-only default permissions and explicit publication behind verification gates.
- npm OIDC/provenance workflow and protected environment references for npm/NuGet. Registry activation remains a maintainer task before the next package release.
- Shared complete-MSVC selection and Docker Desktop Linux verification fallback; locked native builds and reliable watch-smoke cleanup.
- Upgrade notify from 7 to 8.2 to remove the unmaintained instant dependency. WorkspaceSnapshot remains the sole business truth; public APIs and package versions are unchanged.

## Local Evidence (2026-09-18)

- Windows and Docker Linux: rustfmt, strict clippy and 38 Rust tests passed on each platform.
- Windows Node 22.23.2 and 24.21.0: native Open and Watch passed. A fresh npm tarball consumer passed Open; package contents validated.
- Docker Linux Node 24.21.0: freshly built native addon passed Open and Watch against a temporary fixture.
- Windows NuGet: fresh local 0.1.7-publiccheck package passed PackageReference Open and Watch; native, managed, README and license assets verified.
- npm audit: zero vulnerabilities. cargo-audit 0.22.0 with --deny warnings: clean across 109 locked dependencies.
- Gitleaks 8.24.3: no findings in 26 locally reachable commits or 227 Git-visible working files at scan time. This is not a guarantee about unfetched refs or previously distributed artifacts.
- Actionlint passed locally and in the exact configured container. ShellCheck was not run by this gate.
- git diff --check and relative-link validation across 18 changed Markdown files passed.
- Windows/Linux format, strict lint, all 38 tests per platform, workflow lint and metadata/license checks passed again on 2026-09-19.

## Hosted Evidence (2026-09-19)

Final readiness head `b4ed22cc9ade79e572994312d18d621802a4bab5` in [PR #2](https://github.com/teamdoticca/argos/pull/2):

- [verify](https://github.com/teamdoticca/argos/actions/runs/35419156419): passed all six required checks, including Windows/Linux/macOS Rust gates and history/dependency scanning.
- [pack-npm](https://github.com/teamdoticca/argos/actions/runs/35419156531): passed all three native targets and Node smokes.
- [pack-nuget](https://github.com/teamdoticca/argos/actions/runs/35419156540): passed all five native builds, packing and four RID consumer smokes. Intel Mac remains pack-verified only.
- Publication jobs were skipped; no new packages were published.

## Public Launch (2026-09-19)

The owner approved the policies, confirmed monitoring of <fotisgpap@doticca.com> and reported completion of distribution-rights and remaining owner-side checks. This is an owner confirmation, not an independent legal audit. The owner subsequently authorized completion of the public launch.

- PR #2 merged without bypassing required checks at `095593e76f6c2260acb5a79e4142c48711311b30`.
- Repository visibility is public. An anonymous HTTPS clone and unauthenticated raw README request succeeded; description and topics were configured.
- Master protection was read back after the visibility change: PRs, strict up-to-date checks, resolved conversations, administrator enforcement, and no force pushes or deletion. Zero reviewer approvals are required for the sole maintainer.
- Required checks are Rust (windows-latest), Rust (ubuntu-latest), Rust (macos-latest), Package metadata, Workflow lint, and History and dependency scan, bound to GitHub Actions app 15368. Path-filtered package workflows are inspected when relevant, not required on every PR.
- Active ruleset 23691498 prevents updates and deletion of version tags matching `v*` or `[0-9]*`, with no bypass actors configured.
- Dependency vulnerability alerts, Dependabot security updates, secret scanning, push protection and private vulnerability reporting were enabled. Readback confirmed the settings; the initial secret-scanning query returned zero open alerts.

## Future Package Release

Public repository launch does not publish a new version or certify registry artifacts. Before the next package release, verify protected npmjs/nugetorg environments and registry trust policies, choose a new version because 0.1.7 already exists, and validate an OIDC release before revoking the old npm token. No package, version tag or credential was changed during launch. See the [release checklist](../../RELEASING.md).
