# NuGet.org public publish

**Epic:** `m19-nuget-org-publish`
**Status:** see `status.md`

## Goal

Publish the **`Argos`** NuGet package to **nuget.org** with the same discipline as npmjs: explicit `workflow_dispatch` only, exact semver (no CI run-suffix spam), while GitHub Packages remains the auto dogfood feed. Prefer **Trusted Publishing (OIDC)** over long-lived API keys.

## Acceptance criteria

- [x] Package metadata gallery-ready (`PackageReadmeFile`, project URL, tags)
- [x] `pack-nuget.yml` supports `registry`: `github` | `nugetorg` | `both`
- [x] `publish-nuget-org` job: exact base version, `NuGet/login@v1` OIDC, push to nuget.org
- [x] Prep checklist documents Trusted Publishing policy fields (org `teamdoticca`, workflow `pack-nuget.yml`, glob `Argos`)
- [x] Root README .NET install primary path is nuget.org (`dotnet add package Argos`)
- [x] No author code-signing certificate required
- [x] `docs/roadmap.md` and `docs/execution-plan.md` updated on progress
- [ ] First public publish via Actions (`registry=nugetorg`) after `NUGET_USER` secret
- [ ] On completion, move this folder to `docs/done/m19-nuget-org-publish`

## Out of scope

- Multi-RID natives (linux/osx)
- Renaming PackageId away from `Argos`
- Author package signing / Certificates UI
- Closing GitHub Packages dogfood
- npm Trusted Publishing
