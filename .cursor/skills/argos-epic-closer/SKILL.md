---
name: argos-epic-closer
description: Close an Argos epic after acceptance criteria pass; move docs/epics/X to docs/done/X; sync roadmap and execution-plan.
---

# Argos epic closer

## When to use

When an epic's acceptance criteria are met and tests pass.

## Actions

1. Verify acceptance criteria in the epic `README.md`
2. Set `status.md` to `done`
3. Move `docs/epics/<name>/` → `docs/done/<name>/`
4. Update `docs/roadmap.md` status rows and Done section
5. Update `docs/execution-plan.md` (completed steps, next epic)
6. Mark the next epic `in_progress` only if intentionally starting it
