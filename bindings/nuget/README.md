# Argos NuGet package

.NET wrapper over the Argos C ABI (`argos_ffi` cdylib).

## Usage

```csharp
var workspace = await ArgosWorkspace.OpenAsync(repoPath);
var snapshot = workspace.CurrentSnapshot;
await foreach (var change in workspace.WatchAsync())
{
    var affected = workspace.GetAffectedScopes(change);
}
```

## RIDs

Native binaries are expected under:

- `runtimes/win-x64/native/argos.dll`
- `runtimes/linux-x64/native/libargos.so`
- `runtimes/osx-arm64/native/libargos.dylib`

Build the Rust `argos-ffi` crate and copy outputs into those folders before packing.

Watching is always optional — Product 1 (Workspace Intelligence) works without `WatchAsync`.
