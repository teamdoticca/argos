# Design — multi-RID NuGet

## Layout

```
runtimes/win-x64/native/argos_ffi.dll
runtimes/osx-arm64/native/libargos_ffi.dylib
runtimes/osx-x64/native/libargos_ffi.dylib
runtimes/linux-x64/native/libargos_ffi.so
runtimes/linux-arm64/native/libargos_ffi.so
```

Managed: `DllImport("argos_ffi")` unchanged. On Unix the runtime loads `libargos_ffi.so` / `libargos_ffi.dylib`.

## CI

1. **build-native** matrix — one job per RID on a matching (or cross) runner; upload `native-<rid>` with `runtimes/<rid>/native/<file>` shape.
2. **pack** — download all natives, stage under `bindings/nuget/Argos/runtimes/`, `dotnet pack` once, verify nupkg contains required RIDs.
3. **smoke-rid** — restore + run smoke with `-p:RuntimeIdentifier=<rid>` on windows / macos / ubuntu (and arm where available).
4. **publish** — same GitHub Packages + nuget.org Trusted Publishing paths as m19; nuget.org repack reuses staged natives (`-SkipBuild`), never win-only rebuild.

## Local pack

- Host default: `pwsh ./scripts/pack-nuget.ps1` → current host RID only.
- Assemble from prebuilt: `-SkipBuild -StageFrom <dir> -RequireRids win-x64,osx-arm64,linux-x64,...`
