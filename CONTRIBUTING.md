# Contributing to Argos

Argos is maintained by Doticca. Bug reports, documentation improvements and focused pull requests are welcome. Contributions are provided under the repository's [MIT license](LICENSE); do not submit code or data you lack permission to share.

## Before Starting

Read the [architecture](docs/architecture.md), [roadmap](docs/roadmap.md) and [execution plan](docs/execution-plan.md). WorkspaceSnapshot remains the sole business truth. Discuss new behavior in an issue first. Maintainers coordinate product work through the active epic; external contributors do not need to create an epic for a bug report or small fix.

Use English in repository files. Follow the [code of conduct](CODE_OF_CONDUCT.md). For vulnerabilities, use the private channels in [SECURITY.md](SECURITY.md), not a public issue.

## Prerequisites

- Git and Rust via rustup; rust-toolchain.toml selects the tested compiler, rustfmt and Clippy.
- Windows x64: Visual Studio C++ desktop build tools and Windows SDK. PowerShell 7 is required for the scripts.
- Linux: a C compiler/linker and glibc. macOS: Xcode command-line tools.
- Node.js 22 or 24 and npm for the Node binding; .NET SDK 8 or newer for the managed wrapper.
- Docker Desktop with Linux containers is the fallback when Windows cannot run a Linux check.

## Verification

From the repository root:

```powershell
./scripts/verify.ps1
./scripts/verify.ps1 -Docker
```

The Windows script locates a complete MSVC installation rather than trusting the newest Visual Studio directory. The Docker check mounts sources read-only and stores Cargo output in dedicated Docker volumes; it verifies Linux, not Windows or macOS. It downloads the pinned Rust image on first use. It never installs or repairs Visual Studio with administrator privileges.

On a configured Unix host, the equivalent Rust checks are:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

For npm, initialize the Windows developer environment first when applicable:

```powershell
. ./scripts/enter-msvc.ps1
cd bindings/npm
npm ci
npm run build
npm run smoke
npm run smoke:watch
npm pack --dry-run
```

The smoke tests need only the host addon. Publication requires all three addons; check:natives is intentionally stricter than a local build. prepack copies the root license into the package directory; this generated copy is ignored by Git. The package README is maintained separately with absolute documentation links.

For NuGet, run `./scripts/pack-nuget.ps1` from PowerShell 7. This produces a host-RID development package, not a complete public release. The packaging workflow assembles and verifies all five RIDs and runs the existing PackageReference smoke application.

## Pull Requests

Keep changes scoped, include a regression test for behavioral fixes, and describe OS/runtime-specific effects. Run the relevant checks and record any untested platforms. Update user docs for public behavior changes. Maintainers synchronize roadmap and execution-plan progress. Do not include credentials, customer repositories, generated binaries or build output.

CI verifies all pull requests without publishing from forks. Approval and merge are maintainer decisions; there is no guaranteed response time. See [release guidance](docs/RELEASING.md) for versioning and publication.
