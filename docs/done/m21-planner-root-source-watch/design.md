# Design

After collecting conventional dirs + config manifests + `dir_looks_like_source` siblings:

1. If the package root directory contains **source files** (`.cs`, `.ts`, `.tsx`, `.js`, `.mjs`, `.jsx`, `.rs`, `.py`, `.html`, …), ensure `watch` includes `"."`.
2. If `watch` is empty **or** only manifest entries (`.csproj`, `package.json`, `Cargo.toml`, …), ensure `"."` (existing empty fallback, extended).

`"."` means the whole package node root for Focused membership and watch — correct for flat SDK-style .NET and small Electron apps.
