<h1 align="center">CRIA</h1>

<p align="center">
	<a href="README.md">English</a>
	&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
	<a href="README_zh.md">简体中文</a>
</p>

<p align="center" style="color:gray;">
	Cloud-music Rust Integration API — a native Rust client for the NetEase Cloud Music API.
</p>

<p align="center">
    <img src="https://img.shields.io/badge/Language-Rust-orange?logo=rust&logoColor=white" alt="Rust">
    <img src="https://img.shields.io/badge/Runtime-compio-informational" alt="compio">
    <img src="https://img.shields.io/badge/License-MIT-blue?logo=opensourceinitiative&logoColor=white" alt="License">
    <a href="https://crates.io/crates/cria-api"><img src="https://img.shields.io/crates/v/cria-api?logo=rust" alt="crates.io"></a>
    <a href="https://docs.rs/cria-api"><img src="https://img.shields.io/docsrs/cria-api?logo=docs.rs" alt="docs.rs"></a>
    <a href="https://github.com/alpaca-herd/cria/actions/workflows/ci.yml"><img src="https://github.com/alpaca-herd/cria/actions/workflows/ci.yml/badge.svg?branch=develop" alt="CI"></a>
</p>

## Overview

CRIA is the NetEase Cloud Music API client maintained by CNMPlayer. It originates from
[ncm-api-rs](https://github.com/imsyy/ncm-api-rs), a Rust port of
[NeteaseCloudMusicApi Enhanced](https://github.com/NeteaseCloudMusicApiEnhanced/api-enhanced), and keeps its
endpoints in sync with api-enhanced. Instead of running a Node.js HTTP service, an application links CRIA and
calls the endpoints directly:

- about 370 endpoint methods on `ApiClient`, one file per upstream module, parameters passed through `Query`;
- the weapi / eapi / linuxapi / xeapi encryption schemes and plain api requests, including the xeapi
  key registration and session handshake;
- a client-owned session: cookies from `Set-Cookie` are merged automatically, the device ID is
  caller-persisted, and every request sets its own User-Agent and Referer;
- runs on the [compio](https://github.com/compio-rs/compio) runtime and sends requests through an injected
  [`cyper::Client`](https://github.com/compio-rs/cyper), so proxies and connection reuse stay under the caller's control.

CRIA was developed as the networking layer of [CNMPlayer](https://github.com/professor-lee/CNMPlayer) and
split out of that repository with its history preserved. CNMPlayer is its first consumer and exercises the
endpoints it uses against the live service; the remaining endpoints are faithful ports that are not
individually verified.

> CRIA is unofficial and not affiliated with NetEase Cloud Music. Read [DISCLAIMER.md](DISCLAIMER.md) before use.

## Installation

The crate is published on crates.io as **`cria-api`** and imported as **`cria`**:

```toml
[dependencies]
cria-api = "0.1"
cyper = "0.9"
compio = { version = "0.19", features = ["macros"] }
```

To try unreleased changes, depend on the repository instead and pin a revision:
`cria-api = { git = "https://github.com/alpaca-herd/cria", rev = "<commit>" }`.

Building requires Rust 1.95 or newer (the compio stack uses `std::cfg_select!`) and, with the default
`native-tls` feature, the system OpenSSL development package (`libssl-dev` on Debian/Ubuntu, `openssl` on Arch).

## Quick start

```rust
use cria::{ApiClient, NcmError, Query};

#[compio::main]
async fn main() -> Result<(), NcmError> {
    let http = cyper::Client::builder().build()?;
    let api = ApiClient::new(None, http);

    let query = Query::new().param("keywords", "海阔天空").param("limit", "5");
    let response = api.cloudsearch(&query).await?;
    println!("{}", response.body["result"]["songCount"]);
    Ok(())
}
```

A complete version lives in [`examples/cloudsearch.rs`](examples/cloudsearch.rs):
`cargo run --example cloudsearch -- 海阔天空`.

Every endpoint returns `Result<ApiResponse, NcmError>`. `ApiResponse` carries the business `status`, the raw
JSON `body` and the response `Set-Cookie` values; responses are not mapped into typed models, so read fields
from `body` the way the upstream documentation describes them. The endpoint reference is
[`docs/API.md`](docs/API.md) (Simplified Chinese, mirroring the upstream documentation).

## Client contract

- **Runtime.** `ApiClient` wraps `cyper::Client`, which is `!Send`; use it on the compio runtime thread
  that created it. Clones are cheap and share the session.
- **Network configuration.** Proxies, DNS and connection pooling are configured on the injected
  `cyper::Client`; `Query` and request options do not carry proxy settings. Request headers are set by the
  library and do not inherit the client's default headers.
- **Session.** Requests that do not set a cookie through `Query` use the client session and merge the
  response `Set-Cookie` back into it (`Max-Age<=0` or an expired `Expires` deletes the cookie).
  `set_cookie` / `clear_cookie` replace or clear the session; requests issued before the change do not write
  stale cookies back. Read `cookie()` to persist the session and `cookie_value("MUSIC_U")` to test for a login.
  A cookie passed through `Query` applies to that request only.
- **Device identity.** `ApiClient::new` generates a random device ID. Persist `device_id()` and restore it
  with `set_device_id` so the service sees one device across runs. xeapi keys and sessions are kept per client.
- **Timeouts.** One deadline covers connecting, response headers and the whole body:
  `ApiClient::DEFAULT_TIMEOUT` (30 s), adjustable with `set_timeout`. Exceeding it returns `NcmError::Timeout`.

## Cargo features

| Feature | Default | Effect |
| --- | --- | --- |
| `native-tls` | yes | HTTPS through the platform TLS stack (`cyper/native-tls`). |
| `gzip` | yes | Accept and decode gzip-compressed API responses. |
| `brotli` | yes | Accept and decode brotli-compressed API responses. |

With `--no-default-features` the crate still compiles, but it cannot reach HTTPS endpoints unless another
crate in the build enables a TLS backend on `cyper`.

## Project status

- Version `0.1.0`, published on [crates.io](https://crates.io/crates/cria-api); API documentation on [docs.rs](https://docs.rs/cria-api).
- Releases are tagged on GitHub and published to crates.io by CI; see [CHANGELOG.md](CHANGELOG.md) and the
  [GitHub Releases](https://github.com/alpaca-herd/cria/releases).
- The API mirrors the upstream module set; new endpoints follow upstream changes.
- Integration tests use local TCP servers and never contact the live service.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the branch rules, development commands, CI and how to port an
endpoint, and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for community expectations.

## Acknowledgements

CRIA builds on the work of the NeteaseCloudMusicApi projects and the earlier Rust port `ncm-api-rs`.
[ACKNOWLEDGEMENTS.md](ACKNOWLEDGEMENTS.md) lists them together with their license notices.

## License

[MIT](LICENSE). Content and services accessed through the library are not covered by this license; see
[DISCLAIMER.md](DISCLAIMER.md).
