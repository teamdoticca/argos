# Native runtimes

Place platform-specific Argos native libraries here before packing NuGet:

- `win-x64/native/argos.dll`
- `linux-x64/native/libargos.so`
- `osx-arm64/native/libargos.dylib`

Built from `crates/argos-ffi` (`cargo build -p argos-ffi`).
