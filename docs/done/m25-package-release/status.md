# Status

done

The owner authorized completion and release on 2026-09-19. PR #11 aligned all versions and merged as 96bb1686ba90d8aaaf400c2e96cb7da7f4d8bad8 after all release checks passed. Publication-disabled runs [npm 35423085439](https://github.com/teamdoticca/argos/actions/runs/35423085439) and [NuGet 35423086398](https://github.com/teamdoticca/argos/actions/runs/35423086398) passed at that SHA.

The npmjs and nugetorg environments require maintainer doticca approval and allow only master. Self-approval is enabled for the sole-maintainer workflow. Registry Trusted Publishing policies match their workflow and environment; NuGet's package scope is exactly Argos.

Production runs [npm 35423965619](https://github.com/teamdoticca/argos/actions/runs/35423965619) and [NuGet 35423967355](https://github.com/teamdoticca/argos/actions/runs/35423967355) successfully published 0.1.8 using OIDC from the same SHA. Public packages: [npm](https://www.npmjs.com/package/@teamdoticca/argos/v/0.1.8), [NuGet](https://www.nuget.org/packages/Argos/0.1.8).

Fresh isolated Windows consumers installed from the public registries passed npm queries/delta/watch and NuGet OpenAsync/WatchAsync. npm audit signatures verified the registry signature and provenance attestation, whose source commit matches the release SHA. dotnet nuget verify --all passed for the public NuGet repository-signed archive; its nuspec also identifies the release SHA. All three npm addons and five NuGet native RIDs, licenses and READMEs were present.

The legacy argos-github-actions npm token was revoked after successful OIDC publication; the token list is empty. The unused NPM_TOKEN GitHub secret was deleted; only NUGET_USER remains.

Protected tag v0.1.8 points to the published SHA. The public [GitHub release](https://github.com/teamdoticca/argos/releases/tag/v0.1.8) includes changelog, workflow evidence and the actual registry artifacts. SHA256:

```text
7be5f1dba6a080504273c2ed31f18fc44f9b2e1c228d4ac2888b1c9cece9da9a  argos-0.1.8.tgz
91ba33e824f6e44411949a6a01f74b3edbf38eb9be461d702f4a76d6e5c65e3e  Argos.0.1.8.nupkg
```

Hosted NuGet runtime smokes passed for win-x64, linux-x64, linux-arm64 and osx-arm64. Intel Mac remains pack-verified only. npm runtime smokes cover Windows/Linux x64 and Apple Silicon. Public APIs and WorkspaceSnapshot semantics are unchanged. No next epic is started.
