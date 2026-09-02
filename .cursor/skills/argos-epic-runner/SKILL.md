---
name: argos-epic-runner
description: Implement the single Argos epic marked in_progress; read acceptance criteria first; sync roadmap and execution-plan on each step.
---

# Argos epic runner

## When to use

When implementing product work for the active epic.

## Actions

1. Find the epic with `status.md` = `in_progress` (only one)
2. Read its `README.md` and `design.md` plus `docs/architecture.md`
3. Implement only that epic's acceptance criteria
4. Do not open the next epic until the current one is closed
5. After each meaningful step, update `docs/roadmap.md` and `docs/execution-plan.md`
