# 01 — Charter and pack contract

**Status:** planned  
**Epic:** [m15-nuget-pack-ci](./README.md)

## Outcome

Define what “done NuGet” means for Argos and what must never land in git.

## Done NuGet (definition)

A release is consumable when:

1. `dotnet pack` produces `Argos.<version>.nupkg` containing managed `Argos.dll` and at least the **declared** RID natives for that release
2. A smoke project PackageReferences the nupkg and successfully runs `OpenAsync` + reads `CurrentSnapshot` JSON
3. Natives are **not** committed; they are produced by build/copy before pack
4. README / bindings docs list supported RIDs for that version

## Stays out of git

- `runtimes/**/native/*.dll`, `*.so`, `*.dylib`
- `artifacts/nuget/*.nupkg`
- `target/` Rust build outputs

## Explicit answers (carry into impl)

| Topic | Answer |
|-------|--------|
| Feed | Local artifacts first; GitHub Packages next; NuGet.org deferred |
| First RID | win-x64 |
| Natives in git | Never |
| Version line | 0.1.x until public surface or multi-RID stability warrants 0.2 / 1.0 |

## Exit criteria

- [ ] Charter accepted in epic README/design (already drafted — confirm during impl kickoff)
- [ ] `.gitignore` covers natives + artifacts/nuget (add `artifacts/` if missing during impl)
- [ ] No architecture or API change proposed

## Non-goals

Scripts, CI YAML, publishing.
