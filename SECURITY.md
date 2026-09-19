# Security Policy

## Supported Releases

Security fixes target the latest published release. Earlier 0.x versions do not have a backport guarantee. Upgrade to the latest release before reporting an already fixed issue. Argos is an early-stage library, not a sandbox or a security boundary.

## Private Reporting

Do not report exploitable vulnerabilities or credentials in public issues. Use [GitHub private vulnerability reporting](https://github.com/teamdoticca/argos/security/advisories/new) when enabled. If that channel is unavailable, contact the Doticca maintainer at <fotisgpap@doticca.com> with the subject "Argos security report".

Provide the affected version, platform, impact and a minimal reproduction using synthetic data. Do not send live tokens or customer repositories. Allow time for private investigation and coordinate disclosure with the maintainer. There is no guaranteed response time, bounty program or support SLA.

## Trust and Resource Boundaries

Argos reads filesystem metadata and workspace manifests using the caller's permissions. Ignore rules and ownership results are planning hints, not access controls. Do not assume discovered paths, imports or symlinks stay within an authorized root. Constrain untrusted repositories with OS/container permissions, resource limits and an explicit policy in the host application.

Snapshots and diagnostics can contain local paths and project names. Review them before attaching logs. Very large or malicious manifests and filesystem trees may consume significant CPU, memory or watch resources. Run hostile inputs in a restricted process; no comprehensive hostile-input audit is claimed.

Native consumers must obey the [C ABI ownership contract](crates/argos-ffi/src/lib.rs). Serialize calls on a handle and stop enumeration before disposal. Managed and Node bindings do not make arbitrary concurrent native calls safe.

Maintainers must scan full reachable Git history and release artifacts before making the repository public, revoke any exposed credentials, protect the default branch and use short-lived publication credentials. A clean scan reduces risk but is not proof that all secrets or vulnerabilities are absent.
