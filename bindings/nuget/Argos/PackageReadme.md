# Argos

.NET wrapper for the Argos workspace intelligence engine (`argos_ffi`).

## Install

```bash
dotnet add package Argos
```

Requires a **win-x64** host so `runtimes/win-x64/native/argos_ffi.dll` is available (`DllImport("argos_ffi")`).

## Quick start

```csharp
using Argos;

await using var ws = await ArgosWorkspace.OpenAsync(@"Z:\path\to\repo");
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
