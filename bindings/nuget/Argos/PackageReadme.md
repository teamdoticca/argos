# Argos

.NET wrapper for the Argos workspace intelligence engine (`argos_ffi`).

## Install

```bash
dotnet add package Argos
```

Native libraries ship under `runtimes/<rid>/native/` for:

| RID | File |
|-----|------|
| `win-x64` | `argos_ffi.dll` |
| `osx-arm64` | `libargos_ffi.dylib` |
| `osx-x64` | `libargos_ffi.dylib` |
| `linux-x64` | `libargos_ffi.so` (glibc) |
| `linux-arm64` | `libargos_ffi.so` (glibc) |

Publish / run with a matching `-r` / `RuntimeIdentifier`. Managed code uses `DllImport("argos_ffi")`.

## Quick start

```csharp
using Argos;

await using var ws = await ArgosWorkspace.OpenAsync("/path/to/repo");
var snapshotJson = ws.CurrentSnapshot;
var scopesJson = ws.ListScopes();

await foreach (var changeJson in ws.WatchAsync())
{
    var affected = ws.GetAffectedScopes(changeJson);
}
```

`WorkspaceSnapshot` is the sole business truth — open a root, then list nodes/scopes or watch planned scopes.

## Links

- Source: https://github.com/teamdoticca/argos
- Docs: https://github.com/teamdoticca/argos/tree/master/docs
