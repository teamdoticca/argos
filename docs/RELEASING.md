# Releases and Public Launch

Doticca maintains the release process. Repository preparation does not authorize changing visibility, publishing packages, creating tags or rotating credentials.

## Before Making the Repository Public

- Confirm Doticca has the rights to distribute all code and fixtures under MIT. Review references to private projects, historical author emails and organization information.
- Scan all reachable Git history and the working tree for secrets. Review findings privately, rotate any exposed credentials and inspect release artifacts. Do not treat deletion from the latest revision as removal from history.
- Merge the readiness changes and run verify, pack-npm and pack-nuget on GitHub with publication disabled. Local Docker cannot certify macOS or every native RID.
- Protect master (or the current default branch): require PRs, passing checks and resolved conversations; restrict force pushes/deletion. Require review where maintainer staffing permits it. Protect release tags and review workflow changes separately.
- Enable private vulnerability reporting, secret scanning, push protection and Dependabot security updates where the GitHub plan permits. Decide whether Discussions is needed; issues are the default support channel.
- Confirm the conduct/security contact is monitored. Verify CODEOWNERS has write access before requiring owner review.
- After explicit owner approval, change visibility, confirm anonymous clone/docs access and set a useful repository description/topics. A package or GitHub release is not required for public visibility.

## Before the Next Package Release

- Create or verify protected environments named npmjs and nugetorg, restricted to the default branch. Require a maintainer approval for production publication where the GitHub plan supports it.
- Configure npm Trusted Publishing for teamdoticca/argos, workflow pack-npm.yml, environment npmjs. After a successful OIDC release, revoke the old NPM_TOKEN and remove it from GitHub. Do not remove a needed credential before the replacement is verified.
- Confirm the NuGet Trusted Publishing policy for pack-nuget.yml matches the nugetorg environment and default branch; retain only NUGET_USER as the account selector, not a long-lived API key.
- Confirm the packed managed `Argos.dll` retains its public strong-name identity. This is separate from optional NuGet package author signing.

## Versioning

Keep Cargo.toml/Cargo.lock, bindings/npm/package.json/package-lock.json and bindings/nuget/Argos/Argos.csproj aligned. Update CHANGELOG.md. Do not overwrite an existing public version. The next release prepared by this change is 0.1.9; 0.1.8 and earlier versions must not be republished. During 0.x, breaking changes belong in a new minor version and require migration notes; patch releases should remain compatible.

## Publication

1. Merge the reviewed release version and changelog into the default branch after verification succeeds.
2. Run pack-npm and pack-nuget with publish=false. Inspect native matrices and archive checks; record the commit SHA. The npm archive must contain all three addons and both license/readme files; the NuGet archive must contain all five native RIDs, its README and license.
3. Dispatch the appropriate workflow from the same default-branch commit with publish=true and registry=npmjs/nugetorg (or both). Public publication requires the protected environment and default-branch validation gate. npm uses OIDC with provenance; NuGet uses OIDC to obtain a temporary key.
4. Verify registry contents and install each package in a fresh consumer. Record the workflow URLs and any untested platforms.
5. Create an immutable version tag for the published SHA and a GitHub release using the changelog. Attach package hashes and build evidence if distributing assets through GitHub. This step is manual; no automatic GitHub release is claimed.

Package workflows no longer publish automatically on every default-branch push. GitHub Packages remains an explicitly dispatched dogfood feed. There is no token-based fallback for public npm publication if OIDC is unconfigured.

If one registry succeeds and the other fails, keep the successful immutable artifact and retry only the failed publication at the same commit/version. For a bad release, deprecate npm or unlist NuGet as appropriate, publish a new fixed version, and document the incident. Never silently replace artifacts or retarget a published tag.

## Completed Public Launch

On 2026-09-19, the owner confirmed the policies, monitored reporting mailbox and remaining owner-side checks, including distribution rights, then authorized completion of the public launch. PR #2 merged at 095593e76f6c2260acb5a79e4142c48711311b30 after final readiness head b4ed22c passed cross-platform verification and npm/NuGet build matrices. All publication jobs were skipped.

Master protection was applied with owner approval and read back successfully: PRs required; strict required checks Rust (windows-latest), Rust (ubuntu-latest), Rust (macos-latest), Package metadata, Workflow lint, and History and dependency scan, bound to GitHub Actions app 15368; resolved conversations; administrator enforcement; force pushes and branch deletion disabled. Required reviewer approvals are zero because there is one maintainer. Path-filtered package workflows are not required checks because they can be absent on docs-only changes; inspect them for package-relevant PRs.

The repository is public; anonymous HTTPS clone and raw README access were verified. Secret scanning, push protection, dependency vulnerability alerts, Dependabot security updates and private vulnerability reporting are enabled. Active ruleset 23691498 prevents updates/deletion of version tags matching `v*` or `[0-9]*`, with no bypass actors. Initial secret-scanning readback returned zero open alerts; this is point-in-time evidence, not a guarantee that future scans cannot find issues.

Local and hosted evidence is recorded in the [readiness status](done/m23-public-readiness/status.md). History and working-tree secret scans found no leaks in the locally available source; this does not certify unfetched refs, registry artifacts or distribution rights. No CODEOWNERS approval is required for the sole-maintainer workflow. Protected publication environments and registry trust remain next-release checks. No package, version tag or credential was changed during launch.

## Completed 0.1.8 Release

On 2026-09-19, the separately authorized release published npm and NuGet 0.1.8 from 96bb1686ba90d8aaaf400c2e96cb7da7f4d8bad8. Both protected environments require maintainer approval and restrict deployments to master. npm and NuGet Trusted Publishing succeeded without a long-lived token fallback. Fresh public consumers passed Open/query and watch tests; npm signature/provenance and NuGet repository-signature verification passed.

The old argos-github-actions npm token was revoked only after OIDC succeeded, and the unused NPM_TOKEN GitHub secret was removed. NUGET_USER remains an account selector. Protected v0.1.8 points to the published SHA; the [GitHub release](https://github.com/teamdoticca/argos/releases/tag/v0.1.8) contains actual registry archives, SHA256 hashes and workflow links. Full evidence and the Intel Mac pack-only limitation are recorded in [m25 status](done/m25-package-release/status.md). Future releases must use a new version.
