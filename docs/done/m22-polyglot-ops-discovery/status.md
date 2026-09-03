# Status

**status:** done

**updated:** 2026-09-03

## Notes

- Shared walk skip-dirs (`vendor`, `.venv`, `venv`, `__pycache__`, `.tox`, `.mypy_cache`, `Pods`) plus ignore hard-defaults.
- Topology order: php → python → go → gradle → maven → git/workspace → ops → document-root.
- Ops file nodes (`dockerfile`, `compose`, `bicep`, `azure-yaml`); compose watch is the file plus sibling build-context dirs.
- Fixture: `fixtures/mixed-polyglot-ops`.
- Public **Argos 0.1.7** — nuget.org + GitHub Packages; npm `@teamdoticca/argos@0.1.7`
