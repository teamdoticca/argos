# m27 — Ignore access watch noise

**Status:** done

## Goal

Stop non-mutating filesystem access notifications (open, read, close-after-read) from becoming `FileModified` or filling the bounded watch queue. Preserve create, content/metadata modification, delete, rename, topology-change, close-after-write, and genuine overflow/recovery behavior. Unblocks Bassode media-library scans on Linux that currently feedback-loop under `@teamdoticca/argos@0.1.10`.

## Acceptance criteria

- [x] Non-mutating access events are dropped **before** bounded-queue insert
- [x] Direct `map_notify_event` mapping cannot turn those access events into mutations
- [x] `Access(Close(Write))` (and ambiguous `Close(Any)`) still surface as `FileModified`
- [x] Create, modify, delete, rename, topology-change and genuine overflow → `scopeRescan` preserved
- [x] Unit tests cover access classification and queue filtering
- [x] Linux integration: repeated readdir/readFile produce no mutation/topology/rescan (including tiny queue); real writes still detected; mutation overflow still rescans
- [x] Node filesystem-mode + `followConfined` read-only scan settles idle, then detects a real write
- [x] Version prepared as **0.1.11**; changelog and compatibility notes updated in English
- [x] Owner-authorized publish and tag `v0.1.11`

## Non-goals

- Changing notify's inotify `WatchMask` via unsupported Config hooks
- Bassode-side workarounds or SQLite/catalog changes
- Disabling overflow protection
