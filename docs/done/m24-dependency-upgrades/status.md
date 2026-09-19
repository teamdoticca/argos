# Status

done

## Delivered (2026-09-19)

- [PR #10](https://github.com/teamdoticca/argos/pull/10) consolidates Dependabot proposals #3 through #8. The owner authorized merge only after all relevant checks pass and subsequent closure of superseded proposals.
- TOML 1.1 uses document deserialization; existing Cargo discovery tests cover the original failures.
- Locked dependencies: napi 3.12.6, napi-derive 3.6.7, napi-build 2.4.3 and uuid 1.26.1.
- CLI 3 configuration and shared build entry preserve explicit targets and Cargo --locked. Generated bindings preserve the handwritten public Node API; expanded smokes cover snapshots, queries, delta, affected scopes and errors.
- Dependabot groups Cargo napi dependencies. Future CLI major upgrades still require coordinated npm/Cargo review.
- Initial CI exposed missing optional @emnapi lock entries from npm 11.6.2. Regeneration with npm 11.19.0 fixed clean installs, including validation in an empty directory.

## Verification

Implementation head a2cf551aa7d4eb4be6eabff3364c224a9ddca0b6 passed all hosted workflows:

- [verify](https://github.com/teamdoticca/argos/actions/runs/35421342658): Windows/Linux/macOS Rust, metadata, workflow lint and history/dependency scans.
- [pack-npm](https://github.com/teamdoticca/argos/actions/runs/35421342774): three native targets, Node 22/24 API/Watch consumers and archive checks.
- [pack-nuget](https://github.com/teamdoticca/argos/actions/runs/35421342777): five native RIDs, multi-RID package and four consumer smokes. Intel Mac remains pack-verified only.
- Local Windows and Docker Linux formatting, strict clippy and 38 tests per platform passed. Windows Node 22.23.2 API/Watch, npm archive/audit and managed NuGet metadata checks passed.

All publication jobs were skipped. Argos versions remain 0.1.7; no version tags or credentials changed. Branch protections remain intact. Final documentation-head checks remain a merge gate; no next epic is started.
