# 04 — Consumer smoke

**Status:** done  
**Epic:** [m15-nuget-pack-ci](./README.md)  
**Depends on:** [02-build-copy-pack-scripts](./02-build-copy-pack-scripts.md)

## Outcome

Define a minimal smoke project/contract that proves PackageReference works for Product 1 (and optionally Watch).

## Proposed smoke location (impl)

```text
bindings/nuget/smoke/
  Argos.Smoke.csproj
  Program.cs
  nuget.config          # points at ../../../../artifacts/nuget or local feed
```

Or `tests/nuget-smoke/` — pick one folder in impl and keep English README.

## Required checks

1. **Open** — `ArgosWorkspace.OpenAsync(fixtures/small-pnpm)` succeeds
2. **Snapshot** — `CurrentSnapshot` non-empty JSON; contains identity + model/planning sections
3. **Query** — `ListScopes()` or `ListNodes()` returns data for the fixture
4. **Dispose** — clean dispose without hang
5. **Optional Watch** — feature-flagged; poll briefly or skip on CI agents without FS event reliability

## Failure UX proof

- Document expected error when native DLL missing (delete native temporarily in a manual test)
- Smoke must fail CI if Open throws

## Mnemon handoff

Mnemon’s “argos package hosting” brief can copy this smoke pattern: PackageReference + fixture path + Open/Snapshot assert.

## Exit criteria (impl)

- [ ] Smoke runs in CI after pack (same job or dependent job)
- [ ] Uses PackageReference to the packed nupkg (not ProjectReference)

## Non-goals

Full Argos integration test suite; Mnemon repo changes.
