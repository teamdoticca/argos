# 02 — Build / copy / pack scripts (spec only)

**Status:** planned  
**Epic:** [m15-nuget-pack-ci](./README.md)  
**Depends on:** [01-charter-and-pack-contract](./01-charter-and-pack-contract.md)

## Outcome

Specify script responsibilities so a later agent can implement PowerShell (+ optional bash) without inventing policy.

## Proposed layout (to create in impl)

```text
scripts/
  pack-nuget.ps1          # primary (Windows / Mnemon dogfood)
  pack-nuget.sh           # optional later for linux/macos agents
```

Output directory: `artifacts/nuget/` (gitignored).

## PowerShell responsibilities (`pack-nuget.ps1`)

1. **Args:** `-Configuration` (default `Release`), `-Version` (default from csproj or override), `-Rid` (default `win-x64`), `-SkipBuild` optional
2. **Build:** `cargo build -p argos-ffi --release` (or matching configuration)
3. **Locate cdylib:** map RID → output file name:
   - `win-x64` → `argos_ffi.dll` or `argos.dll` (normalize to **`argos.dll`** in destination)
   - Document exact cargo artifact name vs packaged name
4. **Copy:** into `bindings/nuget/Argos/runtimes/<rid>/native/`
5. **Validate:** fail if destination file missing or zero bytes
6. **Pack:** `dotnet pack bindings/nuget/Argos/Argos.csproj -c Release -o artifacts/nuget` with version property if overridden
7. **Print:** full path to nupkg

## Optional bash

Same steps for linux-x64 / osx-arm64 once those RIDs are in scope; not required for first win-x64 PR.

## Exit criteria (impl)

- [ ] One command produces a valid nupkg on a clean Windows machine with Rust + .NET SDK
- [ ] Fails loudly when native missing
- [ ] Does not commit natives

## Non-goals (this brief’s planning scope)

Writing the script files themselves in the planning pass.
