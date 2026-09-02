# AGENTS

Project agents for Argos. Read `docs/roadmap.md` and `docs/execution-plan.md` before substantive work.

## Rules (always on)

| Rule | Purpose |
|------|---------|
| `.cursor/rules/argos-docs.mdc` | Docs lifecycle; progress sync |
| `.cursor/rules/argos-english-only.mdc` | English-only repository files |
| `.cursor/rules/argos-no-version-labels.mdc` | No V1/v1 branding |
| `.cursor/rules/argos-snapshot-truth.mdc` | WorkspaceSnapshot is sole business truth |

## Skills

| Skill | When to use |
|-------|-------------|
| `argos-docs-steward` | Keep docs tree consistent; sync roadmap + execution plan |
| `argos-epic-runner` | Implement the single in_progress epic |
| `argos-epic-closer` | Complete an epic and move it to `docs/done/` |
| `argos-architecture` | Check changes against frozen architecture |

## Hard constraints

- English only in all repository files
- Update `docs/roadmap.md` and `docs/execution-plan.md` on every progress
- Never introduce a second public truth model beside `WorkspaceSnapshot`
- Architecture is frozen — implement, do not pivot product identity
