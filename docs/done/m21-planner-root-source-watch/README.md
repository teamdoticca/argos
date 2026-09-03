# Planner: package-root source watch

**Epic:** `m21-planner-root-source-watch`
**Status:** done

## Goal

Focused consumers (Mnemon) must include dirty files that live at a **package root** (flat .NET / Electron), not only under conventional dirs like `src/` / `components/`.

## Bug

`discover_watch_roots` treats a lone `.csproj` / `package.json` as a non-empty plan and never falls back to `"."`. Root-level `Program.cs`, `DaemonHealth.cs`, `main.js`, etc. are therefore outside Argos scopes → Mnemon Focused shows only packages with conventional subdirs (e.g. `mnemon-web`).

## Acceptance criteria

- [x] Flat .NET package (`.cs` + `.csproj` at node root) plans `watch` containing `"."`
- [x] Package with source subdirs **and** root-level `.cs` still covers root files via `"."` (or equivalent)
- [x] Flat Node package (`package.json` + root `main.js`) plans `"."`
- [x] Packages with only conventional dirs and no root sources keep prior behavior (no forced widen unless needed)
- [x] Unit tests in `argos-core` planner
- [x] Public **0.1.6** on nuget.org / GitHub Packages; this folder moved to `docs/done/`

## Out of scope

- Mnemon Focused UI / membership parser changes
- musl / new RIDs
