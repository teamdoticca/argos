# Argos

Argos is a workspace intelligence engine that discovers repository topology, resolves ignores, plans efficient watch scopes and produces change events for developer tools.

**Mission:** Argos answers a single question: **"What parts of this workspace matter right now?"**

```text
Open Workspace
Build Snapshot
Explain Workspace
Optional Watching
```

## What Argos is

Argos builds a versioned `WorkspaceSnapshot` that represents what matters in a repository, together with explanations, ownership information, scope plans and workspace deltas.

Watching is only a synchronization mechanism for keeping snapshots current.

**Reference consumer:** [Mnemon](https://github.com/teamdoticca) — Argos exists first to solve Mnemon-class problems.

### Two products

1. **Workspace Intelligence** (primary) — Open → `CurrentSnapshot` → explain / list / find owner
2. **Change Tracking** (secondary) — optional `WatchAsync` that advances immutable snapshots

### Core concepts

```text
Workspace → Snapshot → Explain → Scope → Change Stream
```

Not: `Folder → Watch → Events`.

## Quick start

### Rust

```text
cargo build
cargo test -p argos-core
cargo run -p argos-benches -- fixtures/small-pnpm
```

### NuGet pack (win-x64)

```powershell
pwsh ./scripts/pack-nuget.ps1
dotnet run --project bindings/nuget/smoke/Argos.Smoke.csproj -- fixtures/small-pnpm
```

### .NET (NuGet wrapper)

```csharp
var workspace = await ArgosWorkspace.OpenAsync(root);
var snapshot = workspace.CurrentSnapshot; // immutable; pointer only

var scopes = workspace.ListScopes();
var owner = workspace.FindOwner(path);
var explanation = workspace.ExplainPath(path);

await foreach (var change in workspace.WatchAsync())
{
    var affected = workspace.GetAffectedScopes(change);
}
```

Native library: build `argos-ffi` and place under `bindings/nuget/Argos/runtimes/<rid>/native/`.

## Documentation

- [docs/README.md](docs/README.md) — documentation map
- [docs/architecture.md](docs/architecture.md) — frozen system contract
- [docs/roadmap.md](docs/roadmap.md) — epics
- [docs/execution-plan.md](docs/execution-plan.md) — live execution status

## Non-goals

- Build system
- Dependency graph engine
- Package manager
- IDE
- Language server
- Watchman replacement

## Language

All repository files are **English only**.

## License

See repository license when published.
