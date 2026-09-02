# Cursor prompts — m15-nuget-pack-ci

Paste one prompt at a time. Implement **01 → 05** in order. English only. Do not change `WorkspaceSnapshot` or public event schema.

## Prompt — Brief 01 (charter confirmation)

```text
Argos epic m15-nuget-pack-ci brief 01.
Confirm charter in docs/epics/m15-nuget-pack-ci/01-charter-and-pack-contract.md.
Ensure .gitignore covers natives and artifacts/nuget. Update status.md of the brief to done when checklist passes.
No pack scripts yet. English only. Sync docs/roadmap.md and docs/execution-plan.md if status changes.
```

## Prompt — Brief 02 (scripts)

```text
Argos epic m15-nuget-pack-ci brief 02.
Implement scripts/pack-nuget.ps1 per docs/epics/m15-nuget-pack-ci/02-build-copy-pack-scripts.md.
Build argos-ffi, copy win-x64 native as argos.dll into bindings/nuget/Argos/runtimes/win-x64/native/, dotnet pack to artifacts/nuget.
Fail if native missing. Do not commit binaries. Mark brief done; sync roadmap/execution-plan.
```

## Prompt — Brief 03 (CI win-x64)

```text
Argos epic m15-nuget-pack-ci brief 03.
Add GitHub Actions job per docs/epics/m15-nuget-pack-ci/03-ci-win-x64.md.
windows-latest: Rust + .NET 8, test argos-core, run pack-nuget.ps1, upload nupkg artifact.
No NuGet.org publish yet. Mark brief done; sync docs.
```

## Prompt — Brief 04 (smoke)

```text
Argos epic m15-nuget-pack-ci brief 04.
Add bindings/nuget/smoke (or agreed path) per docs/epics/m15-nuget-pack-ci/04-consumer-smoke.md.
PackageReference the packed nupkg; OpenAsync fixtures/small-pnpm; assert CurrentSnapshot.
Wire into CI after pack. Mark brief done; sync docs.
```

## Prompt — Brief 05 (publish)

```text
Argos epic m15-nuget-pack-ci brief 05.
Document/implement GitHub Packages publish path per docs/epics/m15-nuget-pack-ci/05-publish-and-versioning.md.
Keep 0.1.x policy. Document Mnemon PackageReference steps. Optional: push on tag.
When all briefs done, close epic with argos-epic-closer (move to docs/done/, sync roadmap + execution-plan).
```
