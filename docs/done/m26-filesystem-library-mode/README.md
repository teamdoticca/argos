# m26 — Filesystem library mode

**Status:** done

## Goal

Add an opt-in filesystem-only discovery and change-tracking mode for explicitly registered directory roots (for example media libraries). Reuse Argos snapshot and watch infrastructure without software-package topology discovery or software-specific ignore defaults. Preserve existing workspace-mode behavior.

## Acceptance criteria

- [x] `OpenOptions.mode = "filesystem"` opens an explicit root without package/Git/Cargo/.NET discovery
- [x] Scopes and coverage are projected through `WorkspaceSnapshot` only (no second truth model)
- [x] Consumer include/exclude filters with documented semantics; no software hard-default ignores in filesystem mode
- [x] Symlink policies: default `nofollow`; optional `followConfined`; explicit `follow` documented as unconfined
- [x] Watch lifecycle: recursive coverage, rename hints without invented identity, overflow/root-loss → rescan signals
- [x] Bounded event buffering; cancellation/stop does not leak watchers
- [x] Node and .NET API parity; Node example for media-directory open/watch/shutdown
- [x] Deterministic tests for filters, links, recovery, overflow, disposal, and workspace regressions
- [x] Architecture, compatibility, changelog, roadmap and execution-plan updated in English
- [x] Packaged Node addon validation attempted on Debian Bookworm / Node 24; Bookworm-built `.node` verified (Ubuntu CI artifact on Bookworm still unclaimed)

## Non-goals

- Bassode catalog, audio metadata, durable queues, SQLite, Roon polling
- Network-share / SMB / FTP notification support claims without evidence
- Replacing Watchman or guaranteeing exactly-once / gap-free streams
