# Status

**status:** in_progress

**updated:** 2026-09-02

## Notes

- Implemented `discovery::document` providers `document-root` and `guidance-path`.
- Planner watches `.` (or cited file) for those providers.
- Fixtures: `docs-handbook`, `docs-guidance-odd`; unit + integration + Argos self-repo tests green.
- Fix (2026-09-03): prefer shallow document root (`docs/` over dense `docs/done`); skip guidance dirs under an emitted document-root.
- Consumer note: Mnemon Focused parser must map Argos `watch: ["."]` to the scope root prefix (not `docs/.`).
- Remaining: close epic → move to `docs/done/` after NuGet bump for the shallow-root fix.