<p align="center">
	<a href="CHANGELOG.md">English</a>
	&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
	<a href="CHANGELOG_zh.md">简体中文</a>
</p>

# 更新记录

本文件按从新到旧记录 `cria-api` crate 的重要变更。格式遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，版本号遵循[语义化版本](https://semver.org/lang/zh-CN/spec/v2.0.0.html)；1.0 之前，次版本号升级可能包含公开 API 的破坏性变更。`CHANGELOG.md` 以英文记录相同条目。

## [Unreleased]

### 变更

- 文档准确说明来源链：CRIA 由 CNMPlayer 维护，源自 ncm-api-rs，并持续对照 api-enhanced 同步接口。
- 发版以 GitHub 为准：在 `main` 上推送 `vX.Y.Z` tag 后，由 CI 将 crate 发布到 crates.io 并创建对应的 GitHub Release。

## [0.1.0] - 2026-11-17

首个独立 crate 版本，从 CNMPlayer 拆出并保留提交历史。

### 新增

- `ApiClient` 提供约 370 个接口方法，与上游 api-enhanced 模块一一对应、一模块一文件，参数通过 `Query` 传入；回包以带原始 JSON body 的 `ApiResponse` 返回。
- weapi、eapi、linuxapi、xeapi 加密与明文 api 请求，含 xeapi 密钥注册与会话握手。
- 会话归客户端所有：自动合并 `Set-Cookie`，设备 ID 可持久化与恢复，每个请求自行设置 User-Agent 与 Referer。
- 单一请求 deadline（`ApiClient::DEFAULT_TIMEOUT`，可用 `set_timeout` 调整）。
- Cargo feature `native-tls`、`gzip`、`brotli`，默认启用。

[Unreleased]: https://github.com/alpaca-herd/cria/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/alpaca-herd/cria/releases/tag/v0.1.0
