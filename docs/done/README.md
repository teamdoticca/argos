# Done epics

Completed Argos epics. Each folder retains README / design / status for historical reference.

| Epic | Notes |
|------|-------|
| m00-foundation | Repo, docs, agents |
| m01-topology-discovery | Topology + DiscoverySource + confidence |
| m02-filesystem-semantics | PathIdentity / casing / junctions |
| m03-workspace-model | Snapshot sections + versions + state |
| m04-ignore-engine | Hard defaults + nested ignores |
| m05-scope-planner | Scope planning IP |
| m06-diagnostics-api | Explain / list / health / rebuild |
| m07-backend-abstraction | WatchBackend + PlatformCapabilities |
| m08-windows-backend | RDCW via notify |
| m09-linux-backend | inotify via notify |
| m10-macos-backend | FSEvents via notify |
| m11-file-and-workspace-events | Dual streams + compute_delta |
| m12-platform-recovery | Overflow → scoped rescan |
| m13-nuget-wrapper | .NET Argos package |
| m14-benchmark-suite | fixtures + argos-bench |
| m15-nuget-pack-ci | pack script, CI, win-x64 nupkg + smoke |
| m16-monorepo-discovery-parity | Nested npm + .NET discovery |
| m17-typescript-napi | `@teamdoticca/argos` napi |
| m18-document-root-discovery | document-root + guidance-path |
| m19-nuget-org-publish | Public nuget.org |
| m20-nuget-multi-rid | Multi-RID nupkg |
| m21-planner-root-source-watch | Package-root `"."` watch |
| m22-polyglot-ops-discovery | PHP/Python/Go/Gradle/Maven + ops |
