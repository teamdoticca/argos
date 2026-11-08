# Design — filesystem library mode

## Contract

Frozen architecture: Open → Snapshot → Explain → Optional Watching. `WorkspaceSnapshot` remains sole business truth. Filesystem mode answers “what parts of this registered tree matter right now?” via scopes, consumer filters and coverage diagnostics.

## Open options (JSON)

```json
{
  "mode": "workspace" | "filesystem",
  "symlinkMode": "follow" | "nofollow" | "followConfined",
  "include": ["**/*.flac"],
  "exclude": ["**/*.partial"],
  "eventBufferCapacity": 8192
}
```

| Field | Workspace default | Filesystem default |
|-------|-------------------|--------------------|
| `mode` | `workspace` | must be set |
| `symlinkMode` | `follow` | `nofollow` |
| filters | ignored | empty include = all; empty exclude = none |
| `eventBufferCapacity` | backend default | `8192` |

## Discovery

Filesystem mode creates one node:

- `id`: `filesystem-root`
- `provider`: `filesystem-root`
- `kind`: `Package`
- `watch`: `["."]`

No polyglot/document/ops discovery. Ignore engine is empty (no hard-defaults / gitignore). Only consumer filters apply.

## Coverage

Additive `planning.coverage` records unreadable paths, skipped links, outside-root skips, notes and `requiresReconciliation`. Filesystem open always sets `requiresReconciliation` with note `startup-gap` because enumerate-then-watch cannot be gap-free.

Root missing / permission denied at open is a hard error — never a successful empty tree.

## Symlinks

- `nofollow`: do not traverse; diagnose skipped links
- `followConfined`: canonical containment under registered root; cycles de-duped; outside-root skipped
- `follow`: allowed explicitly in filesystem mode; not confined; not race-free

Argos confinement is not authorization for subsequent consumer reads.

## Events

Additive optional `fromPath` / `toPath` / `reason` on file events. Reliable both-path renames emit `fileRenamed`; otherwise create/delete/modify and/or `scopeRescan` with `uncertain-rename`. Overflow and root loss emit `scopeRescan` with reasons; never claim silent completeness.

## Bindings

Same `optionsJson` surface for Node and .NET. No ABI break. Generated napi declarations updated via existing build workflow.
