# Design — m19 nuget.org publish

## Policy (locked)

| Feed | When | Version |
|------|------|---------|
| GitHub Packages | Auto on package-relevant main push | `{base}.{GITHUB_RUN_NUMBER}` |
| **nuget.org** | Explicit `workflow_dispatch` only | Exact `{base}` from csproj (or `v*` tag) |

Auth for nuget.org: **Trusted Publishing (OIDC)** via `NuGet/login@v1`. Secret `NUGET_USER` holds the nuget.org **profile username** (not email) used with the org policy. No long-lived `NUGET_ORG_API_KEY` in the happy path. No author signing certificates.

## Package id

Keep **`Argos`** (same as GitHub Packages / Mnemon). First push creates the public listing under org `teamdoticca`.

## CI shape

1. Existing `pack-win-x64` packs + smoke (version per trigger rules).
2. `publish-github-packages` unchanged for github registry targets.
3. New `publish-nuget-org`:
   - Condition: `workflow_dispatch` + `publish` + registry `nugetorg|both`
   - Checkout + Rust/.NET + **re-pack with exact base** from csproj (ignore CI 4-part artifact for public)
   - `permissions: id-token: write`
   - `NuGet/login@v1` with `user: ${{ secrets.NUGET_USER }}`
   - `dotnet nuget push` to `https://api.nuget.org/v3/index.json`

## Trusted Publishing policy (human)

| Field | Value |
|-------|--------|
| Policy name | `argos-github-actions` |
| Package owner | `teamdoticca` |
| Repository owner | `teamdoticca` |
| Repository | `argos` |
| Workflow file | `pack-nuget.yml` |
| Environment | empty |
| Glob | `Argos` |
| Push scope | new packages and versions |

Private repos may need a successful OIDC login within ~7 days to permanently activate the policy.

## Snapshot / product truth

Packaging only — no change to `WorkspaceSnapshot` contract.
