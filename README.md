# Argos

**Argos answers one question: what parts of this workspace matter right now?**

It is a **workspace intelligence engine** for developer tools. You open a repo root; Argos discovers packages (pnpm/npm/Cargo/.NET/nested Node), resolves ignores, plans **watch scopes**, and exposes an immutable **`WorkspaceSnapshot`** as the single source of business truth. Optional watching keeps that snapshot in sync — it is not “watch a folder and hope.”

```text
Open workspace → Build WorkspaceSnapshot → Explain / List / Own
                         ↓ (optional)
                   WatchAsync → file + workspace events
```

**Not:** `Folder → Watch → Events`.  
**Yes:** `Workspace → Snapshot → Scopes → (optional) Change stream`.

Reference consumer: [Mnemon](https://github.com/teamdoticca/Mnemon) (and similar monorepos).

---

## What you get

| Capability | API surface | Purpose |
|------------|-------------|---------|
| Topology | `ListNodes()` / snapshot `model.nodes` | Packages, imports, solutions discovered under the root |
| Scopes | `ListScopes()` / snapshot `planning.scopes` | Efficient watch roots per package (not whole-repo watch by default) |
| Ownership | `FindOwner(path)` | Deepest node that owns a path |
| Explain | `ExplainPath(path)` | Exists? ignored? watched? which scope? |
| Truth | `CurrentSnapshot` | Immutable JSON snapshot; pointer advances on rebuild |
| Sync | `WatchAsync()` | Optional; emits change events so tools can refresh |

Hard rule: **`WorkspaceSnapshot` is the only public business truth.** Do not invent a parallel “runtime state” model.

---

## Install (.NET / win-x64)

Argos ships as NuGet package **`Argos`** with a native `argos_ffi` RID asset. Feed: **GitHub Packages** (`teamdoticca`).

Every push to `main`/`master` that touches **package-relevant paths** (`crates/`, `bindings/nuget/`, pack script, fixtures, …) publishes a unique version: `{csproj Version}.{GITHUB_RUN_NUMBER}` (e.g. `0.1.2.42`). Docs-only / README-only pushes do **not** publish. Check the latest [Actions](https://github.com/teamdoticca/argos/actions) / [org packages](https://github.com/orgs/teamdoticca/packages) for the exact version.

### `NuGet.config` (consumer)

```xml
<?xml version="1.0" encoding="utf-8"?>
<configuration>
  <packageSources>
    <add key="nuget.org" value="https://api.nuget.org/v3/index.json" />
    <add key="github-argos" value="https://nuget.pkg.github.com/teamdoticca/index.json" />
  </packageSources>
</configuration>
```

Authenticate with a PAT that has `read:packages` (and org SSO if required):

```powershell
dotnet nuget add source https://nuget.pkg.github.com/teamdoticca/index.json `
  --name github-argos `
  --username YOUR_GITHUB_USERNAME `
  --password YOUR_PAT `
  --store-password-in-clear-text
```

### `PackageReference`

```xml
<PackageReference Include="Argos" Version="0.1.2.*" />
<!-- or pin the exact 4-part version from CI, e.g. 0.1.2.42 -->
```

Supported RID today: **win-x64**. Host apps should publish/run as win-x64 so `runtimes/win-x64/native/argos_ffi.dll` is found (`DllImport("argos_ffi")`).

### Local pack (no Packages)

```powershell
pwsh ./scripts/pack-nuget.ps1
# → artifacts/nuget/Argos.<version>.nupkg
```

Point a local feed at `artifacts/nuget` or `dotnet add package` from that folder.

---

## Quick start (.NET)

```csharp
using System.Text.Json;
using Argos;

await using var ws = await ArgosWorkspace.OpenAsync(@"Z:\path\to\repo");
var sample = Path.Combine(@"Z:\path\to\repo", "packages", "app", "src", "index.ts");

// 1) Snapshot is immutable JSON (identity + model + filesystem + planning)
string snapshotJson = ws.CurrentSnapshot;
using var snap = JsonDocument.Parse(snapshotJson);
Console.WriteLine(snap.RootElement.GetProperty("identity").GetProperty("state"));
// expected: "ready"

// 2) Discovered packages / projects
string nodesJson = ws.ListNodes();
// expected (pnpm fixture): non-empty array with package nodes

// 3) Watch plan derived from the snapshot
string scopesJson = ws.ListScopes();
// expected: one scope per package with relative watch roots

// 4) Ownership + explain
string owner = ws.FindOwner(sample);
string explain = ws.ExplainPath(sample);

// 5) Optional watching (scopes come from the snapshot — never empty-scope silent no-op)
await foreach (string changeJson in ws.WatchAsync())
{
    var affected = ws.GetAffectedScopes(changeJson);
    // rebuild / refresh tool state from CurrentSnapshot as needed
}
```

---

## Examples with expected results

### 1) Small pnpm workspace (`fixtures/small-pnpm`)

Layout: root `pnpm-workspace.yaml` + `packages/app` + `packages/lib`.

```powershell
cargo run -p argos-core --example open_scopes -- fixtures/small-pnpm
```

**Expected (illustrative):**

```text
nodes=2
node id=app provider=pnpm .../packages/app
node id=lib provider=pnpm .../packages/lib
scopes=2
scope package=app ... watch=["src", "package.json"]
scope package=lib ... watch=["src", "package.json"]
```

Same via NuGet smoke:

```powershell
dotnet run --project bindings/nuget/smoke -- fixtures/small-pnpm
# SMOKE OK scopes=2 nodes=2
```

### 2) Nested npm without root workspace (`fixtures/nested-npm`)

No root `package.json` / `pnpm-workspace.yaml` — only `src/web/package.json`.

```powershell
cargo run -p argos-core --example open_scopes -- fixtures/nested-npm
```

**Expected:** at least one node with `provider=npm-nested` (id `web`) and a non-empty scope covering `src` / `app` / `package.json`.

### 3) .NET solution stub (`fixtures/dotnet-sln`)

Minimal `.sln` + `.csproj` under `src/App`.

```powershell
cargo run -p argos-core --example open_scopes -- fixtures/dotnet-sln
```

**Expected:** node `App` from `dotnet-sln` / `dotnet-csproj`, scopes non-empty (often includes `App.csproj` and/or package-root `.`).

### 4) Mnemon-class monorepo (real tree)

```powershell
cargo run -p argos-core --example open_scopes -- Z:\Doticca\Mnemon
```

**Expected:** many nodes — nested Node apps (`npm-nested`) plus `.csproj`/`.sln` packages (`dotnet-*`); `scopes > 0`.

Watch smoke (touches a file under a discovered package):

```powershell
cargo run -p argos-backend-windows --example watch_touch -- `
  Z:\Doticca\Mnemon `
  src\frontend\mnemon-web\app\globals.css
```

**Expected:** at least one `FileModified` (or equivalent) for that path. Ignored trees (`.next/`, `node_modules/`, `bin/`, `obj/`) stay out via the ignore engine.

### 5) Snapshot shape (contract)

`CurrentSnapshot` JSON always includes:

```json
{
  "identity": {
    "workspaceId": "...",
    "schemaVersion": 1,
    "snapshotVersion": 1,
    "contentVersion": 1,
    "state": "ready"
  },
  "model": { "nodes": [ /* packages / imports / … */ ] },
  "filesystem": { /* case sensitivity, artifacts, … */ },
  "planning": {
    "ignores": [ /* patterns */ ],
    "scopes": [
      {
        "package": "app",
        "root": ".../packages/app",
        "watch": ["src", "package.json"]
      }
    ]
  }
}
```

Empty `model.nodes` / `planning.scopes` on a real multi-package repo is a **bug**, not an acceptable limitation.

---

## Develop Argos itself

```powershell
# Core tests (includes fixture regression)
cargo test -p argos-core

# Inspect scopes for any root
cargo run -p argos-core --example open_scopes -- <repo-root>

# Pack NuGet locally (win-x64)
pwsh ./scripts/pack-nuget.ps1
```

Workspace layout (high level):

| Path | Role |
|------|------|
| `crates/argos-core` | Discovery, ignore, planner, snapshot |
| `crates/argos-backend-*` | OS watch backends |
| `crates/argos-ffi` | C ABI for managed wrappers |
| `bindings/nuget/Argos` | .NET package |
| `fixtures/` | Synthetic monorepos for tests/smoke |
| `docs/architecture.md` | Frozen system contract |

---

## Documentation

- [docs/README.md](docs/README.md) — documentation map  
- [docs/architecture.md](docs/architecture.md) — frozen contract (`WorkspaceSnapshot` is truth)  
- [docs/roadmap.md](docs/roadmap.md) — epics  
- [docs/execution-plan.md](docs/execution-plan.md) — live status  
- [docs/done/m16-monorepo-discovery-parity/](docs/done/m16-monorepo-discovery-parity/) — nested npm + .NET discovery  

---

## Non-goals

Argos is **not** a build system, package manager, IDE, language server, dependency-graph engine, or a Watchman clone. It does not replace `git status`. Perfect language-aware indexing belongs in consumers (e.g. Mnemon), not here.

---

## Language & license

Repository files are **English only**. Package license: **MIT** (see `Argos.csproj` / package metadata).
