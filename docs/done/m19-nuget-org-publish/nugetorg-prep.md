# nuget.org publish prep — `Argos`

English checklist for public NuGet Gallery listing.

## Why nuget.org

GitHub Packages works for dogfood but needs `read:packages` PAT. Public nuget.org unlocks:

```bash
dotnet add package Argos
```

## Versioning policy

| Feed | When | Version shape |
|------|------|----------------|
| GitHub Packages | Every main push (path-filtered) | `{base}.{run}` e.g. `0.1.4.42` |
| **nuget.org** | Explicit only (`workflow_dispatch`) | Semver **`0.1.4`** — **no** run suffix |

## Trusted Publishing (required)

Create policy on nuget.org → Trusted Publishing:

| Field | Value |
|-------|--------|
| Policy name | `argos-github-actions` |
| Package owner | `teamdoticca` |
| CI/CD | GitHub Actions |
| Repository owner | `teamdoticca` |
| Repository | `argos` |
| Workflow file | `pack-nuget.yml` |
| Environment | *(leave empty)* |
| Glob patterns | `Argos` |
| Scopes | Push → new packages and package versions |

Do **not** register Certificates on the org (author signing not used).

## GitHub Actions secrets

| Secret | Purpose |
|--------|---------|
| `NUGET_USER` | nuget.org **profile username** (not email) for `NuGet/login@v1` |

No long-lived nuget.org API key in the happy path.

## CI publish

Actions → **pack-nuget** → Run workflow:

- `publish`: true
- `registry`: `nugetorg` (or `both`)

## After first publish

- Confirm https://www.nuget.org/packages/Argos
- Root README already points consumers at nuget.org
- Mnemon may use `PackageReference Include="Argos" Version="0.1.4"` from nuget.org (no GitHub feed required)

## Status

**Done.** First public publish: **Argos 0.1.4** — https://www.nuget.org/packages/Argos/0.1.4

```bash
dotnet add package Argos --version 0.1.4
```
