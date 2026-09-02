# Argos documentation

English only. Architecture is frozen — implement against the contract; do not invent a second source of truth.

## Map

| Document | Purpose |
|----------|---------|
| [architecture.md](architecture.md) | Frozen system contract |
| [roadmap.md](roadmap.md) | Epics (current + deferred) |
| [execution-plan.md](execution-plan.md) | Live execution status (update on every progress) |
| [epics/](epics/) | Active epic folders |
| [done/](done/) | Completed epics |

## Epic lifecycle

1. Work only with an epic whose `status.md` is `in_progress`
2. On every meaningful progress, update `roadmap.md` **and** `execution-plan.md`
3. When done: move `docs/epics/<name>/` → `docs/done/<name>/` and sync roadmap + execution plan

Each epic contains:

- `README.md` — goal and acceptance criteria
- `design.md` — design notes aligned to architecture
- `status.md` — `planned` | `in_progress` | `done`

## Progress rule

Incomplete progress that skips roadmap or execution-plan updates is not complete.
