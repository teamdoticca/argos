# Status

in_progress

The owner authorized completion and release on 2026-09-19. PR #11 aligned all versions and merged as 96bb1686ba90d8aaaf400c2e96cb7da7f4d8bad8 after all release checks passed. Publication-disabled runs 35423085439 (npm) and 35423086398 (NuGet) passed at that SHA.

The npmjs and nugetorg environments require maintainer doticca approval and allow only master. Self-approval is enabled for the sole-maintainer workflow. Registry Trusted Publishing policies match their workflow and environment; NuGet's package scope is exactly Argos.

Production runs [npm 35423965619](https://github.com/teamdoticca/argos/actions/runs/35423965619) and [NuGet 35423967355](https://github.com/teamdoticca/argos/actions/runs/35423967355) successfully published 0.1.8 using OIDC from the same SHA. npm's fresh isolated Windows consumer passed queries, delta and watch; all three native addons, license and README were present. npm audit signatures verified the registry signature and provenance attestation.

The legacy argos-github-actions npm token was revoked after successful OIDC publication; the token list is empty. The unused NPM_TOKEN GitHub secret was deleted; only NUGET_USER remains. NuGet public indexing, fresh consumer verification and the immutable tag/GitHub release remain pending. Intel Mac remains pack-verified only.
