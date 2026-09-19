# Design

Preserve WorkspaceSnapshot as the sole business truth. Add contributor-facing documentation and verification around the existing Rust, npm and NuGet surfaces. Reuse existing package smoke programs. Treat locally executed tests, remote CI results and unconfigured administrator settings as separate evidence.

Keep package versions unchanged until a deliberate release. Avoid publishing from forks or arbitrary dispatch refs. Use Docker for Linux checks, not as proof of Windows or macOS behavior.
