# Native runtimes

Place platform-specific Argos native libraries here before packing NuGet:

- `win-x64/native/argos_ffi.dll`
- `linux-x64/native/libargos_ffi.so`
- `osx-arm64/native/libargos_ffi.dylib`

Built from `crates/argos-ffi` (`cargo build -p argos-ffi`).

On Windows the native name must **not** be `argos.dll` — it collides with managed `Argos.dll` on a case-insensitive filesystem. The managed wrapper uses `DllImport("argos_ffi")`.
