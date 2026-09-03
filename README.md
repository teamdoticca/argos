# Argos

**Argos answers one question: what parts of this workspace matter right now?**

Use it when your tool opens a real repo (monorepo or not) and needs to know **which packages exist**, **what is safe to ignore**, **where to watch for changes**, and **who owns a path** — without recursively watching the entire git root or treating `node_modules` / `.next` / `bin` / `obj` as signal.

Argos is a **workspace intelligence** library (not a file-watcher product). You open a root; it builds an immutable **`WorkspaceSnapshot`**. Watching is optional and driven by that snapshot’s scopes.

```text
Open → Snapshot → List nodes / scopes / explain / find owner
              ↘ optional WatchAsync → change events
```

Typical consumers: IDE overlays, repo monitors, agent tooling (e.g. [Mnemon](https://github.com/teamdoticca/Mnemon)).

---

## Problems it solves

| Pain without Argos | With Argos |
|--------------------|------------|
| “Watch the whole monorepo” and drown in noise | Scoped watch roots per package |
| Guessing package roots by hand | Discovers pnpm/npm workspaces, nested `package.json`, Cargo, `.sln`/`.csproj`, … |
| Ignoring `node_modules` / build dirs inconsistently | Ignore engine + planner share one snapshot |
| Events with no business context | Ownership, explain, affected scopes from the same truth |
| Parallel “runtime state” beside the model | **`WorkspaceSnapshot` is the only business truth** |

---

## Add Argos to your .NET project (win-x64)

Package id: **`Argos`** on [nuget.org](https://www.nuget.org/packages/Argos) (managed wrapper + native `argos_ffi` for **win-x64**).

```bash
dotnet add package Argos
```

Or pin a version:

```xml
<PackageReference Include="Argos" Version="0.1.4" />
```

Run / publish your host as **win-x64** so `runtimes/win-x64/native/argos_ffi.dll` is available (`DllImport("argos_ffi")`).

```csharp
using System.Text.Json;
using Argos;

var root = @"Z:\path\to\your\repo";
await using var ws = await ArgosWorkspace.OpenAsync(root);

var snap = JsonDocument.Parse(ws.CurrentSnapshot);
// expected: identity.state == "ready"
var scopesJson = ws.ListScopes();
// expected: length > 0 on a real multi-package repo

await foreach (var changeJson in ws.WatchAsync())
{
    var affected = ws.GetAffectedScopes(changeJson);
    // refresh from ws.CurrentSnapshot when needed
}
```

Dogfood / CI builds also publish to [GitHub Packages](https://github.com/orgs/teamdoticca/packages) as `{version}.{run}` (PAT with `read:packages` required). Public releases use exact semver on nuget.org.

---

## Add Argos to your Node / TypeScript project

Package: **[`@teamdoticca/argos`](https://www.npmjs.com/package/@teamdoticca/argos)** (napi-rs). Supported natives: **win32-x64**, **linux-x64-gnu**, **darwin-arm64**.

```bash
npm install @teamdoticca/argos
```

```ts
import { Workspace, watch } from '@teamdoticca/argos'

const ws = Workspace.open('/path/to/repo')
const snap = ws.currentSnapshot
// expected: snap.identity.state === 'ready'
const scopes = ws.listScopes()
// expected: scopes.length > 0 on a real multi-package repo

for await (const change of watch(ws, AbortSignal.timeout(30_000))) {
  const affected = ws.getAffectedScopes(change)
  // refresh from ws.currentSnapshot when needed
}
ws.close()
```

---

## Snapshot contract (what “truth” looks like)

```json
{
  "identity": { "state": "ready", "snapshotVersion": 1, "contentVersion": 1 },
  "model": { "nodes": [ { "id": "app", "kind": "package", /* … */ } ] },
  "filesystem": { /* case sensitivity, artifacts, … */ },
  "planning": {
    "ignores": [ /* … */ ],
    "scopes": [
      { "package": "app", "root": "…/packages/app", "watch": ["src", "package.json"] }
    ]
  }
}
```

Treat empty `nodes` / `scopes` on a real monorepo as a failure to open usefully — Argos is meant to produce a usable plan.

---

## What to expect on common layouts

### pnpm workspace (two packages)

After `OpenAsync` on a tree like `pnpm-workspace.yaml` + `packages/app` + `packages/lib`:

- **Nodes:** `app`, `lib` (provider `pnpm`)
- **Scopes:** two entries; each typically watches `src` and `package.json`
- **Watch:** edit `packages/app/src/…` → file change events; churn under `node_modules` should not dominate

### Nested Node app (no root workspace file)

Tree with only e.g. `src/frontend/web/package.json`:

- **Nodes:** nested package(s) via `npm-nested`
- **Scopes:** non-empty; often `app` / `src` / `package.json` under that package

### .NET solution / projects

Tree with `.sln` / `.csproj`:

- **Nodes:** project directories (`dotnet-sln` / `dotnet-csproj`)
- **Scopes:** project dirs (source folders and/or `*.csproj`; flat projects may watch `.` at the **package** root)
- **Ignore:** `bin/`, `obj/` stay out of the interesting set

### Mixed monorepo (Node + .NET)

Same `OpenAsync` — expect **both** nested npm packages and .NET projects in `ListNodes()` / `ListScopes()`, then watch only those planned roots.

---

## API cheat sheet

| You want… | Call |
|-----------|------|
| Full truth | `CurrentSnapshot` |
| Discovered packages | `ListNodes()` |
| Where to watch | `ListScopes()` |
| Who owns this file | `FindOwner(path)` |
| Why a path matters / is ignored | `ExplainPath(path)` |
| Live updates | `WatchAsync()` + `GetAffectedScopes(change)` |
| Rebuild without dispose | `RebuildSnapshot()` |

---

## What Argos is not

Not a build system, package manager, IDE, language server, dependency-graph engine, or a drop-in Watchman. It does not replace `git status`. Deep language indexing belongs in **your** product; Argos tells you **which parts of the tree matter** for that work.

---

## Further reading

- [docs/architecture.md](docs/architecture.md) — frozen `WorkspaceSnapshot` contract  
- [docs/README.md](docs/README.md) — full documentation map  

Package license: **MIT**.
