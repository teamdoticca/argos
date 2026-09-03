# Native runtimes

Place platform-specific Argos native libraries here before packing NuGet:

- `win-x64/native/argos_ffi.dll`
- `osx-arm64/native/libargos_ffi.dylib`
- `osx-x64/native/libargos_ffi.dylib`
- `linux-x64/native/libargos_ffi.so`
- `linux-arm64/native/libargos_ffi.so`

Built from `crates/argos-ffi` (`cargo build -p argos-ffi --target <triple>`).

On Windows the native name must **not** be `argos.dll` — it collides with managed `Argos.dll` on a case-insensitive filesystem. The managed wrapper uses `DllImport("argos_ffi")` (Unix resolves `libargos_ffi.so` / `.dylib`).

Linux natives in this package are **glibc** builds; musl is deferred.
