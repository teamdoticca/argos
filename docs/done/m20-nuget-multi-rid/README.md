# NuGet multi-RID natives

**Epic:** `m20-nuget-multi-rid`
**Status:** see `status.md`

## Goal

Ship **one** Argos NuGet package with native libraries for Windows, macOS, and Linux so consumers (especially Mnemon) can run Argos watch without a Windows-only RID default.

## RID matrix

| RID | Native filename | Cargo target | Priority |
|-----|-----------------|--------------|----------|
| `win-x64` | `argos_ffi.dll` | `x86_64-pc-windows-msvc` | required |
| `osx-arm64` | `libargos_ffi.dylib` | `aarch64-apple-darwin` | required |
| `linux-x64` | `libargos_ffi.so` | `x86_64-unknown-linux-gnu` (glibc) | required |
| `osx-x64` | `libargos_ffi.dylib` | `x86_64-apple-darwin` | shipped (pack + RequireRids; Intel smoke deferred) |
| `linux-arm64` | `libargos_ffi.so` | `aarch64-unknown-linux-gnu` (glibc) | shipped |

**Packaging choice:** one multi-RID nupkg (standard `runtimes/<rid>/native/` layout). Managed API stays `DllImport("argos_ffi")` — .NET resolves `libargos_ffi.so` / `.dylib` on Unix.

**Linux note:** glibc (`*-unknown-linux-gnu`) only for this milestone; musl (`linux-musl-*`) deferred.

## Acceptance criteria

- [x] Single published nupkg includes at least `win-x64` + `osx-arm64` + `linux-x64` natives
- [x] `osx-x64` and `linux-arm64` included when CI runners allow; otherwise explicit defer note in README
- [x] Pack script + CI assemble one multi-RID package; publish to GitHub Packages like today
- [x] Smoke per RID: OpenAsync → non-empty ListScopes → watch sees a file change (win / linux-x64 / linux-arm64 / osx-arm64; osx-x64 pack-verified)
- [x] win-x64 regression green
- [x] `bindings/nuget/README.md` lists RIDs, exact filenames, pack/publish, glibc note
- [x] Version bumped; package id/version recorded on close — **Argos `0.1.5`** (nuget.org) / dogfood `0.1.5.*`
- [x] `docs/roadmap.md` and `docs/execution-plan.md` updated on progress
- [x] On completion, move this folder to `docs/done/m20-nuget-multi-rid`

## Out of scope

- Mnemon PackageReference / RID default changes
- Windows ARM
- musl / alpine natives
- Author NuGet code-signing certificates
