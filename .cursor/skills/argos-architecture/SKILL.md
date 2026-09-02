---
name: argos-architecture
description: Review Argos changes against docs/architecture.md; reject scope creep and second truth models; enforce snapshot-first API philosophy.
---

# Argos architecture

## When to use

When designing APIs, reviewing PRs, or deciding whether a feature belongs in Argos.

## Actions

1. Read `docs/architecture.md`
2. Apply the Mnemon filter: Would Mnemon use this?
3. Reject features that turn Argos into a generic folder watcher
4. Reject public second state models beside `WorkspaceSnapshot`
5. Prefer Open → Snapshot → Explain → Optional Watching
6. Enforce Non-goals from the root README
