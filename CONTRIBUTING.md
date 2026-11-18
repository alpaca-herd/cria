<p align="center">
	<a href="CONTRIBUTING.md">English</a>
	&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
	<a href="CONTRIBUTING_zh.md">简体中文</a>
</p>

# Contributing to CRIA

CRIA is the NetEase Cloud Music API client maintained by CNMPlayer. It originates from ncm-api-rs, a Rust port
of NeteaseCloudMusicApi Enhanced, and keeps its endpoints in sync with api-enhanced. This document covers the
branch and pull-request rules, the development environment, the repository layout, how to port an endpoint,
and the formatting and CI conventions.

## Branch and pull request rules

- **Open pull requests against `develop`.** `develop` is the integration branch.
  Never open a pull request directly against `main`.
- **`main` is the release branch and is maintained by the repository owner
  only.** The maintainer opens pull requests from `develop` into `main`; do not
  propose changes to `main` yourself.
- **One PR, one topic.** Each pull request against `develop` must deal with a
  single subject. If you have several unrelated changes, open several pull
  requests — a PR that mixes topics will be rejected. Major contributions are
  the exception, at the maintainer's discretion.
- **Never put credentials in a PR or an issue.** Cookies such as `MUSIC_U`, phone numbers, passwords, device
  IDs and captured request bodies from a real account must be redacted.

## Development environment

Rust 1.95 or newer. `rust-toolchain.toml` requests the current stable toolchain, so with rustup `cargo` uses
the latest installed stable. The crate uses edition 2024.

The locked dependencies require Rust 1.95: `compio-driver 0.12.4` uses
[`std::cfg_select!`](https://doc.rust-lang.org/stable/std/macro.cfg_select.html), stabilized in 1.95.
Edition 2024 alone would only require 1.85; it does not establish the real dependency MSRV.

System build dependencies — the same list CI installs on `ubuntu-24.04`, needed by the default `native-tls`
feature:

```bash
# Debian / Ubuntu
sudo apt install -y pkg-config libssl-dev
```

```bash
# Arch Linux
sudo pacman -S --needed pkg-config openssl
```

Commands:

```bash
cargo build
cargo test --locked --all-targets
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo run --example cloudsearch -- 海阔天空   # needs network
```

Tests must not contact the live service. Exercise request construction, headers, cookies and timeouts against
local TCP servers, as the tests in `src/request/tests.rs` do; verify live behavior with a throwaway program and
describe the result in the pull request instead of committing it.

## Repository layout

| Path | Contents |
| --- | --- |
| `src/lib.rs` | Crate root: module declarations and the public re-exports (`ApiClient`, `ApiResponse`, `CryptoType`, `RequestOption`, `Query`, `NcmError`). |
| `src/request.rs` | `ApiClient`: request construction per encryption scheme, headers, the client-owned cookie jar and xeapi state, deadlines and response decoding. |
| `src/crypto.rs` | weapi, eapi, linuxapi and xeapi encryption and decryption. |
| `src/api/` | One file per upstream module, each adding methods to `ApiClient` through `impl ApiClient`; `mod.rs` registers them and defines `Query`. |
| `src/util/` | Upstream constants (`config.rs`), cookie parsing and the session jar (`cookie.rs`), device identifiers and IP helpers. |
| `src/error.rs` | `NcmError` and the crate `Result` alias. |
| `examples/` | Runnable examples. |
| `docs/API.md` | Endpoint reference (Simplified Chinese, following the upstream documentation). |
| `.github/` | CI (`workflows/ci.yml`), the release workflow (`workflows/release.yml`) and its helper scripts. |
| Root documents | `README.md` / `README_zh.md` (two language variants kept in parallel), `CHANGELOG*.md`, `CONTRIBUTING*.md`, `CODE_OF_CONDUCT*.md`, `DISCLAIMER*.md`, `ACKNOWLEDGEMENTS*.md`, `LICENSE` (MIT). |

## Porting an endpoint

New and changed endpoints follow the upstream project; CRIA does not invent endpoints of its own.

1. Start from the upstream file `module/<name>.js` in
   [NeteaseCloudMusicApi Enhanced](https://github.com/NeteaseCloudMusicApiEnhanced/api-enhanced) and name the
   Rust file and method after it (`src/api/<name>.rs`, `ApiClient::<name>`).
2. Keep the upstream semantics: URL, encryption scheme, request body fields, defaults, cookie overrides and any
   response post-processing. Note deliberate differences in a comment that explains why.
3. Add the `/// 对应 Node.js module/<name>.js` doc line like the existing endpoints, and register the module
   in `src/api/mod.rs` under the matching section.
4. Document the method in `docs/API.md` in the same format as its neighbours.
5. In the pull request, link the upstream file and commit you ported and state how you verified it.

Changes to shared request behavior (`src/request.rs`, `src/crypto.rs`, `src/util/`) affect every endpoint;
add tests for them and keep the client contract in the README accurate.

## Intentionally omitted upstream modules

CRIA intentionally does not expose these `api-enhanced` modules as independent `ApiClient` methods:

- `decrypt`: an upstream server-side debugging helper; CRIA exposes its local EAPI decrypt implementation instead.
- `register_checktoken_v2` / `register_checktoken_v3`: anti-cheat token acquisition is internal to the request layer.
- `register_xeapikey`: xeapi public-key registration is internal to the request layer.
- `register_neapikey`: requires the separate upstream `neapi` encryption and key-rotation subsystem, which CRIA does not implement.
- `scrobble_v1`: requires the upstream NCBL encryption format, which CRIA does not implement.
- `song_url_match` / `song_url_ncmget`: third-party song-unblocking helpers, not API client endpoints.

These omissions are deliberate scope boundaries, not forgotten modules. Reconsider them only with a corresponding design and verification plan.

## Code conventions

- **Import explicitly.** Name the items you use; do not add glob imports.
- **Keep visibility minimal.** Items stay private or `pub(crate)` unless they are part of the public API; do not
  widen visibility only for tests.
- **Keep tests in sibling files.** Put tests for `foo.rs` in `foo/tests.rs` (declared with
  `#[cfg(test)] mod tests;`); production modules do not embed test modules or test-only helpers.
- **Public API changes are breaking changes.** Renaming or removing a public item, or changing a signature,
  needs a version bump discussed with the maintainer.

## Formatting and CI

Formatting is `rustfmt` with the repository's `rustfmt.toml`: `edition = "2024"`, `max_width = 100`,
`use_field_init_shorthand = true`.

Linting is `cargo clippy` with `-D warnings`; `clippy.toml` relaxes `too-many-arguments-threshold` to 8 and
`type-complexity-threshold` to 300.

CI (`.github/workflows/ci.yml`) runs on pull requests targeting `main` / `develop`, on pushes to `develop`, and
as the gate of the release workflow. Every job runs on `ubuntu-24.04` and installs the system dependencies
listed above:

- `build-and-test` runs `cargo check` and `cargo test` (`--locked --all-targets`) on stable, with default
  features and with `--no-default-features`.
- `quality` runs on stable with the `rustfmt` and `clippy` components: `cargo fmt --all -- --check`, clippy with
  `-D warnings` for `--all-features` and `--no-default-features`, and `cargo doc --no-deps` with
  `RUSTDOCFLAGS=-D warnings`.
- `package` runs `cargo package --locked`, which builds the crate from the packaged sources exactly as
  crates.io would receive them.

The equivalent local commands, job by job:

```bash
# build-and-test
cargo check --locked --all-targets
cargo test --locked --all-targets
cargo check --locked --all-targets --no-default-features
cargo test --locked --all-targets --no-default-features

# quality
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo clippy --locked --all-targets --no-default-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps

# package
cargo package --locked
```

`Cargo.lock` is committed so CI and contributors build the same dependency versions; update it in the same
pull request as a dependency change.

## Changelog

A pull request that changes the crate's behavior or public API adds a line under `## [Unreleased]` in both
`CHANGELOG.md` and `CHANGELOG_zh.md`, in the matching `Added` / `Changed` / `Fixed` / `Removed` subsection.
Changes that do not reach crate users (tests, CI, internal refactors) need no entry.

## Releases

GitHub is the source of truth for versions, tags and release notes; crates.io only receives what CI
publishes from a tag. Releases are cut by the maintainer:

1. On `develop`, open a pull request that bumps `version` in `Cargo.toml` (run `cargo check` so
   `Cargo.lock` follows) and renames `## [Unreleased]` to `## [X.Y.Z] - YYYY-MM-DD` in both changelogs,
   adding a fresh empty `## [Unreleased]` and updating the comparison links at the bottom.
2. Merge `develop` into `main` through a pull request.
3. Tag the merged `main` commit and push the tag:
   `git tag -a vX.Y.Z -m "vX.Y.Z" origin/main && git push origin vX.Y.Z`. A pre-release version such as
   `0.2.0-rc.1` uses the tag `v0.2.0-rc.1`.

The tag starts `.github/workflows/release.yml`:

- `verify` requires the tag to equal `v` + the `Cargo.toml` version, the tagged commit to be on `main`, and
  both changelogs to have a non-empty section for the version; it also records whether crates.io already has
  the version.
- `gate` runs the whole CI workflow on the tagged commit.
- `semver checks` runs `cargo-semver-checks` against the newest release on crates.io and fails when the
  version bump is too small for the API changes.
- `publish` runs `cargo publish --locked` in the `crates-io` environment. It authenticates with crates.io
  Trusted Publishing, so the repository stores no crates.io token.
- `github-release` creates the GitHub Release from the changelog sections (English, then Chinese) and marks
  pre-release versions as such.

Re-running a failed release is safe: a version already on crates.io is not published again and an existing
GitHub Release is kept. A version on crates.io can never be replaced; fix mistakes with a new version (and
`cargo yank` if needed). To rehearse a release, run the Release workflow manually (`workflow_dispatch`) on a
branch: it runs the same checks plus `cargo publish --dry-run` and publishes nothing.
