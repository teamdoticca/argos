# Design — m16 monorepo discovery parity

## Root cause

Discovery only consulted **root** manifests: `pnpm-workspace.yaml`, root `package.json` workspaces, root `Cargo.toml` workspace, `.workspace`, `.gitmodules`, plus a root-heuristic for lone `package.json`/`Cargo.toml`. Mnemon has **none** of those at the git root: nested Next apps (`src/frontend/mnemon-web/package.json`, …) and many `.csproj`/`.sln` under `src/`. Providers that ran returned no package nodes → empty topology → empty `ListScopes()` → `WatchAsync` registered zero roots (silent no-op).

## Providers (after fix)

| Provider | Trigger |
|----------|---------|
| `pnpm` | Root `pnpm-workspace.yaml` |
| `npm` | Root `package.json` workspaces |
| `npm-nested` | Any nested `package.json` (skips root; skips `node_modules`/`.next`/…) |
| `cargo` | Root Cargo workspace |
| `dotnet-sln` / `dotnet-csproj` | `.sln` Project lines + walk for `.csproj` |
| `git-submodules` | `.gitmodules` |
| `workspace-import` | `.workspace` imports |
| `heuristic` | Root package/cargo/csproj/sln when nothing else |
| `fallback-root` | Explicit last resort: one `workspace-root` node (Low confidence) so Open never yields silent empty watch |

## Minimal node set (current monorepo parity)

1. Nested Node packages (`package.json` under tree)
2. .NET projects (`.csproj`, preferably via `.sln`)
3. Existing cargo / root pnpm / npm workspace / imports / submodules
4. Explicit `fallback-root` only when still empty

## Scope planner

- Prefer conventional dirs (`src`, `app`, …) and config files at package root
- Include `*.csproj` at package root for .NET
- If still empty: watch `"."` = **package** root only (ignore-aware); never prefer whole-git-root recursion when better package nodes exist

## Safe fallback

`fallback-root` is documented, Low confidence, and ignore-aware. Happy path remains scoped package roots.
