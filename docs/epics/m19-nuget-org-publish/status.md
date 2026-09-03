# Status

**status:** in_progress

**updated:** 2026-09-03

## Notes

- Kickoff after m18 close.
- Trusted Publishing policy created on nuget.org (org `teamdoticca`, glob `Argos`, workflow `pack-nuget.yml`).
- CI + metadata + README wired.
- **Blocker for first push:** Actions secret `NUGET_USER` = nuget.org profile username (not email).
- Then: Actions → pack-nuget → publish=true, registry=nugetorg.