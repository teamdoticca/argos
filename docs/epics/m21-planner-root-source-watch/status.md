# Status

**status:** in_progress

**updated:** 2026-09-03

## Notes

- Repro: Mnemon working tree dirty in Tray/Api/Electron/Mcp; Focused showed only ~7 `mnemon-web` files.
- Cause: planner watch = manifests only for flat package roots.
- Fix implemented in `discover_watch_roots`: ensure `"."` when root has sources or watch is manifest-only.
- Unit tests green (4).
- Release line **0.1.6** (nuget.org cannot reuse 0.1.5).
