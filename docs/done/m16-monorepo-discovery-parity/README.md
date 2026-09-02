# Monorepo discovery parity

**Epic:** `m16-monorepo-discovery-parity`
**Status:** see `status.md`

## Goal

Make Open → Snapshot → Scope Planner produce **non-empty, useful scopes** for common monorepo shapes (nested npm without workspace root, .NET `.sln`/`.csproj`), so `WatchAsync` receives file events for git-visible source roots. Reference consumer: Mnemon.

## Acceptance criteria

- [x] Root-cause documented (why Mnemon yielded empty nodes)
- [x] Nested `package.json` discovery without root workspace file
- [x] `.sln` / `.csproj` discovery as package nodes
- [x] Scope planner emits watch roots for those packages (package-scoped `.` when needed; not silent empty)
- [x] `fixtures/small-pnpm` + Argos self-repo still discover scopes
- [x] Fixture coverage for nested npm + stub .NET layout
- [x] Smoke: `OpenAsync(Mnemon)` → `ListScopes()` length > 0; watch sees edit under discovered package
- [x] NuGet patch bump for Mnemon PackageReference (`0.1.2`)
- [x] `docs/roadmap.md` and `docs/execution-plan.md` updated on progress
- [x] On completion, move this folder to `docs/done/m16-monorepo-discovery-parity`

## Out of scope

- Changing Mnemon public overlay contracts
- Replacing `git status`
- Watchman / shared daemon
- Perfect language-aware indexing
