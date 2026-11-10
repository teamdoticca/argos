# Status

in_progress

Opened 2026-11-10 after Bassode confirmed Linux filesystem-mode read storms map access notifications to `FileModified` and overflow under `@teamdoticca/argos@0.1.10`. Target patch line **0.1.11** (unpublished until owner approval).

Implementation on `fix/ignore-access-watch-noise`: `is_non_mutating_access` drops open/read/close-read before bounded-queue insert; `map_notify_event` keeps `Close(Write|Any)` as `FileModified`. Unit tests, Linux integration tests, and `scripts/validate-access-noise-bookworm.sh` added. Manifests at 0.1.11.
