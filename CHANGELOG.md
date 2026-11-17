<p align="center">
	<a href="CHANGELOG.md">English</a>
	&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
	<a href="CHANGELOG_zh.md">简体中文</a>
</p>

# Changelog

All notable changes to the `cria-api` crate are recorded here, newest first. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and versions follow
[Semantic Versioning](https://semver.org/spec/v2.0.0.html); before 1.0, a minor version bump may break the
public API. `CHANGELOG_zh.md` carries the same entries in Simplified Chinese.

## [Unreleased]

### Changed

- The documentation states the provenance accurately: CRIA is maintained by CNMPlayer, originates from
  ncm-api-rs and keeps its endpoints in sync with api-enhanced.
- Releases are cut from GitHub: a `vX.Y.Z` tag on `main` publishes the crate to crates.io through CI and
  creates the matching GitHub Release.

## [0.1.0] - 2026-11-17

First release as a standalone crate, split out of CNMPlayer with its history preserved.

### Added

- `ApiClient` with about 370 endpoint methods, one file per upstream api-enhanced module, parameters passed
  through `Query`; responses are returned as `ApiResponse` with the raw JSON body.
- weapi, eapi, linuxapi and xeapi encryption and plain api requests, including the xeapi key registration
  and session handshake.
- A client-owned session: `Set-Cookie` values are merged into the session automatically, the device ID can
  be persisted and restored, and every request sets its own User-Agent and Referer.
- A single per-request deadline (`ApiClient::DEFAULT_TIMEOUT`, adjustable with `set_timeout`).
- Cargo features `native-tls`, `gzip` and `brotli`, enabled by default.

[Unreleased]: https://github.com/alpaca-herd/cria/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/alpaca-herd/cria/releases/tag/v0.1.0
