# Status

in_progress

## Current

The public master is green. Dependabot PRs #4, #6, #7 and #8 fail because CLI options, napi macro/runtime versions and TOML document parsing require coordinated migration. PRs #3 and #5 propose napi-build and uuid patch upgrades.

## Implemented

- Use toml 1.1 document deserialization instead of parsing a standalone Value. Existing Cargo discovery tests cover the original failures.
- Upgrade napi runtime/macros together. Locked resolution uses napi 3.12.6, napi-derive 3.6.7 and napi-build 2.4.3; uuid is 1.26.1.
- Upgrade the npm CLI to the 3.10 line, migrate binaryName/targets configuration and use a shared build entry that forwards target flags before Cargo's --locked argument.
- Regenerate native bindings without changing the handwritten public Node wrapper or its TypeScript contract. Extend existing smoke coverage for snapshot, queries, delta, affected scopes and errors.
- Group coupled Cargo napi dependencies in Dependabot. CLI major upgrades still require coordinated review across npm and Cargo.

## Local Evidence (2026-09-19)

- Windows and Docker Linux: formatting, strict workspace clippy and all 38 tests passed per platform.
- Windows Node 22.23.2: freshly built napi 3 addon passed expanded API and Watch smokes; explicit target forwarding and npm archive checks passed.
- npm audit found zero vulnerabilities. Package version/license metadata and managed NuGet archive checks passed. Local cargo-audit is unavailable; the required hosted audit remains the security gate.
- Public Argos versions remain 0.1.7; no packages, tags or credentials changed.

## Remaining

PR #10 first hosted run passed Rust on all three OSes, history/Cargo audit and all NuGet native/consumer jobs. npm installation exposed missing optional @emnapi lock entries produced by npm 11.6.2. Regenerating with CI's npm 11.19.0 added those entries; clean npm ci passed in an empty directory. Full hosted verification must rerun on the repaired lockfile.

The owner approved one consolidated PR, merge only after every relevant hosted check passes, and closing PRs #3 through #8 as superseded. Verify all host OS checks, Node 22/24 native smokes and NuGet native/consumer matrices before merge. Keep branch protections and publication gates intact.
