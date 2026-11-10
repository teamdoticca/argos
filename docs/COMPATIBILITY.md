# Compatibility and Lifecycle

Argos is an early-stage 0.x library. Snapshot and binding APIs may change between minor releases; patch releases are intended for compatible fixes. Read the changelog before upgrading and pin a package version where reproducibility matters. The snapshot schemaVersion is distinct from the package version; consumers should reject unsupported schemas and tolerate additional JSON fields.

## Supported Distribution

| Surface | Runtime / platform | Validation |
| --------- | -------------------- | ------------ |
| NuGet | net8.0; win-x64 | Native build and PackageReference open/watch smoke |
| NuGet | net8.0; linux-x64, linux-arm64 (glibc) | Native build and PackageReference open/watch smoke |
| NuGet | net8.0; osx-arm64 | Native build and PackageReference open/watch smoke |
| NuGet | net8.0; osx-x64 | Cross-built native and archive presence; no Intel Mac runtime smoke |
| npm | Node.js 22 / 24; win32-x64, linux-x64-gnu, darwin-arm64 | Native build and Node open/watch smoke |
| Rust | Toolchain in rust-toolchain.toml | Source builds on Windows, Linux and macOS CI hosts |

The npm manifest currently permits Node >=18 for historical compatibility. Node 18/20 are end-of-life and are not supported maintenance targets; use Node 22 or 24. Architecture must match the running process, not just the host OS. No public crates.io publication is promised by the Rust workspace manifests.

Linux binaries are built on the workflow's Ubuntu runner. Older glibc distributions are not guaranteed; use a compatible modern distribution or build from source. Alpine/musl, Windows ARM64, npm Linux ARM64 and npm Intel macOS are not shipped. Network shares, unusual filesystems, container bind mounts and file-watcher limits can behave differently from local disks. No exhaustive minimum-OS compatibility certification is claimed. NativeAOT, single-file extraction and trimming are not currently validated configurations.

## Lifecycle

Open a workspace, read its immutable snapshot, then optionally watch. An empty repository may yield a fallback root rather than package nodes. Parsing a snapshot in .NET requires disposing the resulting JsonDocument separately from the workspace.

Use one watcher/enumerator per workspace. In .NET, pass a CancellationToken to WatchAsync and await enumeration completion before Dispose/DisposeAsync. Cancellation may throw OperationCanceledException. OpenAsync performs its open synchronously and checks cancellation before opening; it does not interrupt an in-progress scan. Serialize calls yourself; do not race reads, rebuilds or disposal against watch polling.

In Node, pass an AbortSignal to watch, await termination, then call close in a finally block. The helper polls roughly every 50 ms; cancellation is cooperative. Ending iteration runs watchStop. Do not close the workspace while an iterator is still active.

Topology events can rebuild the snapshot. Events are filesystem notifications, not a durable, ordered audit log or a guaranteed one-event-per-edit stream. Reconcile against CurrentSnapshot after topology/recovery changes. Consumers must handle missed/coalesced events and periodic reconciliation where correctness requires it.

Non-mutating access notifications (open, read, close-after-read) are dropped before the bounded watch queue and do not become `fileModified`. Close-after-write remains a modification hint. Overflow protection still applies to retained mutation events.

### Filesystem mode

Pass `"mode":"filesystem"` in open options JSON to observe an explicitly registered directory without software topology discovery. Defaults: `symlinkMode` is `nofollow`; include/exclude are empty (all files eligible, no consumer excludes). Workspace mode defaults and behavior are unchanged when mode is omitted or `"workspace"`.

Additive snapshot fields (`identity.mode`, `planning.filters`, `planning.coverage`, optional file-event `fromPath`/`toPath`/`reason`) keep `schemaVersion` at 1; consumers must tolerate additional JSON fields. Filesystem open sets `planning.coverage.requiresReconciliation` because enumerate-then-watch cannot guarantee a gap-free startup. Overflow, root disappearance and uncertain renames emit `scopeRescan` (with `reason` when available) rather than claiming silent completeness.

Symlink confinement in Argos is observation policy only — consumers must independently validate identity/confinement before opening files. Network shares, bind mounts and Linux ARM64 npm natives are not claimed by this mode without separate evidence.

Linux npm natives built on Ubuntu CI may require a newer glibc than Debian Bookworm; validate the packaged `.node` artifact on the target distro or rebuild locally.

**Bookworm evidence (2026-11-08):** `scripts/validate-filesystem-bookworm.sh` built `argos.linux-x64-gnu.node` inside `debian:bookworm-slim` (glibc 2.36) with Node 24.22.2, opened a filesystem-mode root on Docker named volume `argos-fs-bookworm-media`, and observed watch events (`BOOKWORM_OPEN_OK` / `BOOKWORM_WATCH_OK`). That verifies a **Bookworm-built** packaged addon, not an Ubuntu-CI artifact loaded on Bookworm. Network-share / Windows bind-mount notification delivery is still unclaimed.

**Access-noise evidence (m27):** `scripts/validate-access-noise-bookworm.sh` opens filesystem mode with `symlinkMode:"followConfined"` and a small `eventBufferCapacity`, runs repeated read-only tree scans expecting idle watch polls (`ACCESS_NOISE_IDLE_OK`), then writes a file and expects a mutation event (`ACCESS_NOISE_WRITE_OK`).

## Troubleshooting

- DllNotFoundException: select the matching NuGet RID, ensure argos_ffi is present with its OS dependencies, and check process bitness. Do not rename the native DLL to Argos.dll.
- Node addon load error: verify the supported platform, architecture and installed package contents. Local builds stage only one addon; check:natives requires all three for publication.
- MSVC LNK1104 for msvcrt.lib: use scripts/enter-msvc.ps1 or scripts/verify.ps1. A Visual Studio installation containing only onecore libraries is insufficient. Install the C++ desktop workload through Visual Studio Installer if no complete installation exists.
- Linux GLIBC version error: use a newer compatible distribution or rebuild the native binding locally.
- Missing watch events: inspect ListScopes, ignore rules and permissions; check inotify/resource limits and retry on a local disk. Docker validation does not certify Windows-host bind-mount event delivery.
