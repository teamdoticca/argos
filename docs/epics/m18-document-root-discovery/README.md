# Document root discovery

**Epic:** `m18-document-root-discovery`
**Status:** see `status.md`

## Goal

During the initial topology scan, Argos must discover important **document root(s)** by evidence (not a fixed `docs/` name) and, because it already opens agent guidance files, also surface **other important folders and files** those files name — rules, skills, workflows, configs, etc. — as snapshot nodes / watch scopes. Markdown is one class of “what matters,” not the only class.

## Acceptance criteria

- [x] Document-root provider scores candidate directories during discovery (signals, not a fixed path name)
- [x] Hardcoding only `docs/` as the happy path is rejected; name aliases may boost score but never gate discovery
- [x] Provider **reads** agent guidance files (`AGENTS.md` and peers) and extracts path hints for **directories and files of any relevant type** (not `.md`-only); existence required
- [x] Extracted paths seed/boost `document-root` and/or emit `guidance-path` nodes when not already covered by package topology
- [x] Winning document root(s) and guidance-important paths appear in the snapshot with provider identity and confidence
- [x] Scope planner emits ignore-aware watch for those nodes (`.` under a doc/guidance dir, or cited files)
- [x] Fixtures: non-`docs` planning tree; classic `docs/` when it is real; guidance pointer to oddly named docs tree; guidance pointer to non-md surface (e.g. `.cursor/rules`); no false win inside `node_modules` / duplicate of an existing package root
- [x] Argos self-repo: Open → scopes include planning tree and guidance-cited agent surfaces (watch plan covers those roots)
- [x] Existing package discovery (`small-pnpm`, nested npm, .NET, cargo) regressions stay green
- [x] `docs/roadmap.md` and `docs/execution-plan.md` updated on progress
- [ ] On completion, move this folder to `docs/done/m18-document-root-discovery`

## Out of scope

- Watching the entire git root as the default strategy
- Full-text / semantic indexing or LLM interpretation of guidance prose
- Replacing package discovery when guidance mentions a package path already owned as a `Package` node
- Mnemon UI polling or overlay contract changes
- Treating every scattered unreferenced `.md` as its own node
- Hard requirement that every repo has a document root (empty is valid when evidence is weak)
