# Public repository readiness

Prepare Argos for external users and contributors without changing its architecture or publishing packages.

## Acceptance Criteria

- MIT license text is present and shipped in npm and NuGet packages.
- Contribution, conduct, security, support, compatibility and release guidance are discoverable.
- Pull requests run Rust formatting, lint and tests across supported host operating systems.
- Release workflows use least privilege, immutable action references and explicit public publication gates.
- Local Windows validation selects complete MSVC tools; Linux fallback uses Docker Desktop with isolated output.
- Dependency and history-secret checks are run; remaining GitHub/registry administrator actions are recorded honestly.

Changing repository visibility, committing, publishing and rotating credentials require separate authorization.
