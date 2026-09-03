# Argos execution plan

Live status. Update on every meaningful progress.

## Current

- **Active epic:** [m18-document-root-discovery](epics/m18-document-root-discovery/)
- **Status:** in_progress — core discovery + planner + tests green; ready to close
- **Last update:** 2026-09-03 — Fix: prefer shallow `docs/` over `docs/done|roadmap`; Mnemon Focused must treat watch `.` as scope root



## Completed

### m00-foundation (done)

- Local repo, GitHub, English docs, agents/rules/skills

### m01–m06 Workspace Intelligence (done)

- Topology discovery through diagnostics/query APIs

### m07–m12 Change Tracking platform (done)

- Backend abstraction, OS backends, events, recovery

### m13–m14 Packaging & benchmarks (done)

- NuGet C# wrapper; benchmarks + fixtures

### m15-nuget-pack-ci (done)

Path: [docs/done/m15-nuget-pack-ci](done/m15-nuget-pack-ci/)

- Pack/CI for win-x64; native `argos_ffi.dll`; GitHub Packages dogfood

### m16-monorepo-discovery-parity (done)

Path: [docs/done/m16-monorepo-discovery-parity](done/m16-monorepo-discovery-parity/)

- Nested npm + .NET discovery; Mnemon scopes; NuGet `0.1.2`

### m17-typescript-napi (done)

Path: [docs/done/m17-typescript-napi](done/m17-typescript-napi/)

- [x] `argos-napi` + `@teamdoticca/argos`
- [x] win / linux-x64 / darwin-arm64 CI + smoke/watch
- [x] GitHub Packages + **npmjs.org** public `0.1.2`

## Next

1. **m18-document-root-discovery** — scored document roots + guidance-cited important folders/files (not md-only)
2. Optional: npm Trusted Publishing (OIDC) and revoke long-lived `NPM_TOKEN`
3. Optional: MCP wrapper epic (`npx` / Cursor tools on `@teamdoticca/argos`)
4. Mnemon PackageReference dogfood (NuGet)
5. linux-x64 / osx-arm64 NuGet RID matrix

## Blockers

None.
