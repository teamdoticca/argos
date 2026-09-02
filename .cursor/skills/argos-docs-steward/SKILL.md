---
name: argos-docs-steward
description: Keep Argos docs tree consistent; sync roadmap and execution-plan on every progress; reject orphan epics and non-English docs.
---

# Argos docs steward

## When to use

Whenever documentation, epic status, roadmap, or execution-plan changes.

## Actions

1. Ensure epic folders live under `docs/epics/` or `docs/done/` only
2. Each epic has `README.md`, `design.md`, `status.md`
3. On any progress, update `docs/roadmap.md` and `docs/execution-plan.md`
4. Reject `V1`/`v1` branding and non-English documentation
5. Reject public second-truth models beside `WorkspaceSnapshot`
