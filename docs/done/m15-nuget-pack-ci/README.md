# NuGet Pack & CI

**Epic:** `m15-nuget-pack-ci`  
**Status:** see `status.md`  
**Depends on:** [m13-nuget-wrapper](../m13-nuget-wrapper/) (done)

## Goal

Make Argos a **consumable NuGet package** with native RID assets, a reproducible build→copy→pack path, and CI that produces artifacts Mnemon can dogfood — without changing the frozen architecture.

**One-line outcome:** clear pack/CI contract so Mnemon’s Argos monitor work depends on a real nupkg, not fragile ProjectReference + manual native copy.

## Problem

m13 shipped the C# wrapper (`ArgosWorkspace`, DllImport, package metadata), but:

- `bindings/nuget/Argos/runtimes/<rid>/native/` has docs only — no `argos.dll` / `.so` / `.dylib`
- No reproducible build → copy → `dotnet pack` path
- No CI pack/publish workflow
- Mnemon’s upcoming working-tree monitor work **blocks on** a consumable package

## Acceptance criteria

- Design documented in `design.md` with explicit answers to feed / RID / commit / versioning
- Briefs 01–05 specify implementation order without requiring architecture changes
- `docs/roadmap.md` and `docs/execution-plan.md` reference this epic
- On completion of **implementation** (follow-up), move this folder to `docs/done/m15-nuget-pack-ci` and sync roadmap + execution plan

## Non-goals

- Implementing pack scripts / CI in this planning pass
- Publishing a nupkg in this planning pass
- Changing Argos public API, event schema, or `WorkspaceSnapshot`
- NPM / TypeScript bindings
- Mnemon repository changes
- Committing native binaries to git

## Briefs

| # | Brief | Outcome (later) |
|---|-------|-----------------|
| 01 | [01-charter-and-pack-contract](./01-charter-and-pack-contract.md) | Done NuGet definition; what stays out of git |
| 02 | [02-build-copy-pack-scripts](./02-build-copy-pack-scripts.md) | PowerShell (+ optional bash) responsibilities |
| 03 | [03-ci-win-x64](./03-ci-win-x64.md) | CI job shape, artifacts, failure conditions |
| 04 | [04-consumer-smoke](./04-consumer-smoke.md) | Smoke project for Open/Snapshot (+ optional Watch) |
| 05 | [05-publish-and-versioning](./05-publish-and-versioning.md) | Feed, semver, Mnemon consume steps |

## Handoff

Next agent implements briefs **01 → 05 in order**. Do not start CI publish before local pack + smoke pass on win-x64.
