# Argos architecture

**Status:** frozen

## Mission

Argos answers a single question: **What parts of this workspace matter right now?**

Topology discovery, filesystem semantics, ignore resolution, scope planning, diagnostics, and watching are different ways to answer that same question.

## System contract

> The WorkspaceSnapshot is the contract of the system. All public APIs, diagnostics, change tracking and integrations are projections of the current or a future snapshot.

### Immutable snapshot rule

- `WorkspaceSnapshot` is an **immutable value object**. It never mutates.
- `CurrentSnapshot` is only a **pointer** on the workspace to the latest snapshot instance (`#42` → `#43`).
- New state always means a **new** snapshot object.

```text
Workspace
 └── CurrentSnapshot ──► WorkspaceSnapshot #42   (immutable)

later:

Workspace
 └── CurrentSnapshot ──► WorkspaceSnapshot #43   (new immutable instance)
```

### No second truth model

Do not grow public business state beside the snapshot (`WorkspaceRuntimeState`, `WatchState`, `PlannerCache`, …). Runtime caches may exist only as implementation details.

`WorkspaceSnapshot = truth`.

## Snapshot shape

Four formal sections plus identity metadata:

```json
{
  "identity": {
    "workspaceId": "4f3b4f65-...",
    "schemaVersion": 1,
    "snapshotVersion": 42,
    "previousSnapshotVersion": 41,
    "contentVersion": 17,
    "previousContentVersion": 16,
    "state": "ready"
  },
  "model": {
    "nodes": []
  },
  "filesystem": {
    "pathIdentity": {},
    "caseSensitivity": {}
  },
  "planning": {
    "ignores": [],
    "scopes": []
  }
}
```

### State

- `ready` — normal operation
- `recovering` — e.g. after platform overflow recovery
- `degraded` — e.g. broken imports / missing submodules

### Versions

- `schemaVersion` — JSON/API contract version
- `snapshotVersion` — increments for every new snapshot instance (including rebuilds)
- `contentVersion` — increments only when business content changes

## Lifecycle

```text
Open Workspace
  → Topology Discovery
  → Filesystem Semantics
  → Workspace Model
  → Ignore Engine
  → Scope Planner
  → WorkspaceSnapshot
  → Diagnostics Ready
  → (Optional) Watch Start
  → File Events + Workspace Events
  → Incremental Snapshot N+1
  → Dispose
```

Invariant: **Open → Build Snapshot → Explain → Optional Watching**

Watchers synchronize an existing snapshot; they do not discover the workspace after the fact.

## Product formula

```text
Argos = Topology Discovery
      + Filesystem Semantics
      + Ignore Resolution
      + Scope Planning
      + Watching (per-OS backends behind abstraction)
```

- **Product 1 — Workspace Intelligence** (primary)
- **Product 2 — Change Tracking** (secondary)

Scope Planner is the IP. Backends are swappable.

## Event streams

Two first-class streams:

1. **File Events** — `file_modified`, `file_created`, `file_deleted`, `file_renamed`, `scope_rescan`, …
2. **Workspace Events** — `workspace_updated`, `package_added`, `package_removed`, …

`argos_compute_delta` uses content-version semantics so rebuild-without-change is not a business delta.

## Platform stance

Windows, Linux, and macOS are equal citizens. Filesystem Semantics come before planning. Backend Abstraction comes before OS implementations.

## Reference consumer

**Mnemon.** Feature filter: *Would Mnemon use this?*

Consumer profiles:

- A — Workspace inspection (VS Code, Cursor)
- B — Incremental indexing (Mnemon)
- C — Monitoring only (build systems)

## Non-goals

- Build system
- Dependency graph engine
- Package manager
- IDE
- Language server
- Watchman replacement

## Core value test

Epics m01–m06 alone remain a useful product without OS backends, NuGet, or benchmarks.
