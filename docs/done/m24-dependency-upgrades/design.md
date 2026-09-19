# Design

Use TOML document deserialization rather than value parsing. Upgrade the napi runtime, procedural macros and CLI as one compatible set, preserving the handwritten Node wrapper and public TypeScript contract. Keep WorkspaceSnapshot as the only business truth. Reuse existing integration tests and package smokes; retain locked dependency builds and explicit publication gates.
