# npmjs.org publish prep — `@teamdoticca/argos`

English checklist. Do this once m17 CI is green on win/linux/darwin-arm64.

## Package scope (important)

GitHub Packages npm **requires** the scope to match the GitHub org that owns the repo (`teamdoticca/argos` → **`@teamdoticca/argos`**).  
Publishing `@doticca/argos` with `GITHUB_TOKEN` fails with `403 … installation does not exist`.

Use **`@teamdoticca/argos`** on both GitHub Packages and npmjs (create npm org `teamdoticca` or grant publish on that scope).

## Why npmjs

GitHub Packages works for dogfood but requires `read:packages` PAT. Public **npmjs** unlocks:

```bash
npm install @teamdoticca/argos
```

## One-time human setup

1. **npm account** that can publish under scope `@teamdoticca`
   - Create/join npm org **`teamdoticca`** (recommended) and claim scope `@teamdoticca`
   - Enable 2FA on the publishing account
2. **GitHub Actions secret** on `teamdoticca/argos`:
   - Name: `NPM_TOKEN`
   - Value: npm Automation token with publish to `@teamdoticca/argos`
3. Confirm **`@teamdoticca/argos`** is free on npmjs

## Versioning policy

| Feed | When | Version shape |
|------|------|----------------|
| GitHub Packages | Every main push (path-filtered) | `{base}-ci.{run}` e.g. `0.1.4-ci.42` |
| **npmjs** | Explicit only (`workflow_dispatch`) | Semver **`0.1.4`** (same base as NuGet/Cargo) — **no** `-ci.*` spam |

## Repo wiring

- [x] Package id `@teamdoticca/argos`
- [x] `prepublishOnly` = native presence check (not `napi prepublish` — that tried to create GitHub Releases and failed CI)
- [x] `pack-npm.yml` dual registry (`github` / `npmjs` / `both`)
- [x] First npmjs publish + README install without GitHub registry

## CI publish

Actions → **pack-npm** → Run workflow:

- `publish`: true  
- `registry`: `npmjs` (or `both`)

## After first npmjs publish

```bash
npm install @teamdoticca/argos
```

**Status:** first public publish done (`0.1.2` on npmjs.org). Later: prefer Trusted Publishing (OIDC) and revoke long-lived `NPM_TOKEN` when ready.
