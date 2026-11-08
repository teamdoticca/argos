# Status

done

Filesystem library mode shipped in public **0.1.10** from `1fe1aa74db5711281475d5363231d996dd056757` ([PR #29](https://github.com/teamdoticca/argos/pull/29)).

Acceptance criteria passed: opt-in `mode: "filesystem"`, path filters, coverage/symlink policies, bounded watch + rename/recovery signals, Node/.NET parity, tests, Bookworm packaged-addon script evidence, English docs. Workspace-mode defaults unchanged; `WorkspaceSnapshot` remains sole business truth.

Dry-run packs then owner-authorized publish succeeded:

- npm: [42515546888](https://github.com/teamdoticca/argos/actions/runs/42515546888) → [@teamdoticca/argos@0.1.10](https://www.npmjs.com/package/@teamdoticca/argos/v/0.1.10)
- NuGet: [42515550295](https://github.com/teamdoticca/argos/actions/runs/42515550295) → [Argos 0.1.10](https://www.nuget.org/packages/Argos/0.1.10)

Protected tag [v0.1.10](https://github.com/teamdoticca/argos/releases/tag/v0.1.10) points at the published SHA. No next epic started.
