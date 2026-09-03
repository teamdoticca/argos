# Status

**status:** in_progress

**updated:** 2026-09-03

## Notes

- Decision: **one multi-RID nupkg** via CI matrix (native per RID) + assemble job.
- Required RIDs: `win-x64`, `osx-arm64`, `linux-x64`. Also ship `osx-x64` + `linux-arm64` when runners succeed.
- Linux natives are **glibc** (`*-unknown-linux-gnu`); musl deferred.
- Local win-x64 pack + open/watch smoke green for `Argos.0.1.5.nupkg`.
- Pending: push → CI matrix smoke → GitHub Packages (and optional nuget.org dispatch).
