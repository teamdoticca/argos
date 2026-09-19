# Package release 0.1.8

## Acceptance Criteria

- Align Cargo, npm and NuGet versions at 0.1.8 and finalize the changelog.
- Protect npmjs and nugetorg publication environments and verify registry OIDC trust.
- Merge only after all release checks pass; run publication-disabled package validation on the release commit.
- Publish both public packages from the same commit and verify fresh consumers.
- Create immutable v0.1.8 tag and GitHub release with evidence; retire the old npm credential only after verified OIDC publication.
- Preserve public APIs and record Intel Mac's pack-only verification limitation.
