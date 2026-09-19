# Status

in_progress

## Implemented

- MIT license and Doticca attribution; verified license/readme inclusion in npm and NuGet archives.
- Contributor, conduct, security, support, compatibility and release guidance; issue and PR templates; Dependabot configuration.
- Committed-lockfile policy, pinned Rust toolchain, strict cross-platform format/lint/test workflow, version/archive checks, dependency audits and full-history secret scanning.
- Immutable action revisions and scanner images, credential-free checkouts, read-only default permissions and explicit publication behind verification gates.
- npm OIDC/provenance workflow and protected environment references for npm/NuGet. Registry activation remains a maintainer task.
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

## Hosted Evidence (2026-09-19)

PR [#2](https://github.com/teamdoticca/argos/pull/2), implementation commit `964b0c8843515fa31c678396303c5534ec74ef3a`:

- [verify](https://github.com/teamdoticca/argos/actions/runs/35418017626): passed Windows/Linux/macOS Rust gates, metadata, workflow lint and history/dependency scanning.
- [pack-npm](https://github.com/teamdoticca/argos/actions/runs/35418017812): passed all three native targets and Node smokes.
- [pack-nuget](https://github.com/teamdoticca/argos/actions/runs/35418017729): built all five RIDs, packed successfully and passed Windows x64, Linux x64/ARM64 and macOS ARM64 consumer smokes. Intel Mac remains pack-verified only.
- All public and GitHub Packages publication jobs were skipped. No merge or visibility change was performed.

## Remaining Launch Gates

On 2026-09-19, the owner approved the MIT license, Code of Conduct, Security Policy and Support guidance, confirmed monitoring of <fotisgpap@doticca.com>, and reported completing the remaining owner-side checks, including distribution rights. This is an owner confirmation, not an independent legal audit. The owner identified themselves as the sole maintainer and authorized master protection and completion of the readiness PR. No merge, publication or visibility change is performed in this preparation step.

Master protection was applied and read back successfully on 2026-09-19: PRs required, zero required reviewer approvals for the sole maintainer, strict up-to-date checks bound to GitHub Actions (app 15368), resolved conversations, administrator enforcement, and no force pushes or branch deletion. Required contexts are Rust (windows-latest), Rust (ubuntu-latest), Rust (macos-latest), Package metadata, Workflow lint, and History and dependency scan. Path-filtered package workflows are not required contexts because docs-only changes may skip them.

On 2026-09-19, the current working tree passed Windows and Docker Linux format, strict lint and all 38 Rust tests per platform again. Workflow lint, package metadata/license checks and diff whitespace checks also passed. The owner authorized a dedicated branch, commit, push and pull request for non-publishing GitHub CI; merge, publication and visibility changes remain unauthorized.

- Review and approve PR #2 before merge; require passing checks on the final reviewed revision. Hosted implementation checks are green; Intel Mac remains pack-verified, not runtime-certified.
- Owner-side policy, contact and distribution checks are confirmed complete. No second-reviewer or CODEOWNER approval is required for the sole maintainer.
- Default-branch protection is verified. Release-tag protection and available GitHub security features still need remote verification; owner completion statements are not evidence of those settings.
- Protected publication environments and registry trust policies must be verified before the next package release; no new package release is needed just to make the repository public.
- Verify an OIDC release before revoking the old npm token; use a new release version because public 0.1.7 already exists.
- Explicit owner approval is required for visibility changes, publication, tags and credential rotation. None of those operations was performed.

The epic remains in_progress until the remote launch gates are verified. See [release checklist](../../RELEASING.md).
