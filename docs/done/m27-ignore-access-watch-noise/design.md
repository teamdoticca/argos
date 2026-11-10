# Design — ignore access watch noise

## Problem

`notify` 8.2 on Linux registers `OPEN` and `CLOSE_WRITE`. Argos 0.1.10 mapped every `EventKind::Access(_)` to `FileModified` and enqueued those events into the bounded watch queue. Read-only scans (`readdir` / `readFile`) therefore look like mutations, overflow the queue, emit `scopeRescan`, and drive consumer rescan loops.

## Approach

Single choke point: [crates/argos-backend/src/notify_map.rs](../../../crates/argos-backend/src/notify_map.rs).

1. **Pre-queue filter** in `OverflowHandler::handle_event`: drop non-mutating access `Ok(Event)` values without `try_send`, so they cannot consume capacity or set overflow.
2. **Defense in depth** in `map_notify_event`: access noise returns no domain events; close-after-write maps to `FileModified`.

## Classification

| notify kind | Action |
|-------------|--------|
| `Access(Read)`, `Access(Open(_))` | Drop |
| `Access(Close(Read\|Execute\|Other))` | Drop |
| `Access(Any)`, `Access(Other)` | Drop |
| `Access(Close(Write))`, `Access(Close(Any))` | Keep → `FileModified` |
| `Create` / `Remove` / `Modify` | Unchanged |
| `EventKind::Other` | Unchanged → `scopeRescan` (`platform-other`) |
| `EventKind::Any` | Keep → `FileModified` |
| Queue full on kept events / `force_overflow` | Unchanged → `scopeRescan` (`overflow`) |

Cannot strip `OPEN` from notify's inotify mask through public `Config`; dropping before enqueue is the supported fix.

## Snapshot truth

No second public truth model. Events remain hints; `WorkspaceSnapshot` stays sole business truth. Consumers still reconcile on real `scopeRescan`.
