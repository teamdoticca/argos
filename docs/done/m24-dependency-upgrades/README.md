# Dependency upgrade compatibility

Resolve the six initial Dependabot proposals without changing Argos package versions or public APIs.

## Acceptance Criteria

- TOML document parsing works with toml 1.1 and existing Cargo workspace discovery tests pass.
- napi, napi-derive and the npm CLI are upgraded together; locked builds, generated bindings and Open/Watch consumers remain compatible.
- uuid and napi-build patch updates are included and verified.
- Windows/Linux Rust gates and supported hosted package matrices pass without publication.
- Dependabot groups compatible napi changes to avoid independent incompatible major upgrades.
- Roadmap, execution plan and evidence remain synchronized; no credentials or registry releases are changed.
