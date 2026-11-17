<p align="center">
	<a href="CONTRIBUTING.md">English</a>
	&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
	<a href="CONTRIBUTING_zh.md">简体中文</a>
</p>

# Contributing to CRIA

CRIA is a Rust port of the NeteaseCloudMusicApi Enhanced endpoints. This document covers the branch and
pull-request rules, the development environment, the repository layout, how to port an endpoint, and the
formatting and CI conventions.

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
| `.github/` | CI. |
| Root documents | `README.md` / `README_zh.md` (two language variants kept in parallel), `CONTRIBUTING*.md`, `CODE_OF_CONDUCT*.md`, `DISCLAIMER*.md`, `ACKNOWLEDGEMENTS*.md`, `LICENSE` (MIT). |

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

CI (`.github/workflows/ci.yml`) runs on pull requests targeting `main` / `develop` and on pushes to `develop`.
Every job runs on `ubuntu-24.04` and installs the system dependencies listed above:

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

Version bumps and publishing are not part of a normal pull request: the maintainer bumps the version in
`Cargo.toml` and publishes the `cria-api` crate to crates.io.
