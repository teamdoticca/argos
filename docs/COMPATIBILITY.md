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

## Troubleshooting

- DllNotFoundException: select the matching NuGet RID, ensure argos_ffi is present with its OS dependencies, and check process bitness. Do not rename the native DLL to Argos.dll.
- Node addon load error: verify the supported platform, architecture and installed package contents. Local builds stage only one addon; check:natives requires all three for publication.
- MSVC LNK1104 for msvcrt.lib: use scripts/enter-msvc.ps1 or scripts/verify.ps1. A Visual Studio installation containing only onecore libraries is insufficient. Install the C++ desktop workload through Visual Studio Installer if no complete installation exists.
- Linux GLIBC version error: use a newer compatible distribution or rebuild the native binding locally.
- Missing watch events: inspect ListScopes, ignore rules and permissions; check inotify/resource limits and retry on a local disk. Docker validation does not certify Windows-host bind-mount event delivery.
