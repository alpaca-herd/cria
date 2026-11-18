<p align="center">
	<a href="CONTRIBUTING.md">English</a>
	&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
	<a href="CONTRIBUTING_zh.md">简体中文</a>
</p>

# 参与 CRIA 贡献

CRIA 是 CNMPlayer 维护的网易云音乐 API 客户端，源自 ncm-api-rs（NeteaseCloudMusicApi Enhanced 的 Rust 移植），并持续对照 api-enhanced 同步接口。本文说明分支与 PR 规则、开发环境、仓库结构、接口移植方式，以及格式与 CI 约定。

## 分支与 PR 规则

- **PR 请提交到 `develop`。** `develop` 是集成分支，不要直接向 `main` 提 PR。
- **`main` 是发布分支，仅由仓库所有者维护。** 由维护者从 `develop` 向 `main` 发起 PR，请勿自行向 `main` 提交改动。
- **一个 PR 只做一件事。** 每个指向 `develop` 的 PR 只处理一个主题；若有多个无关改动，请拆成多个 PR，混合主题的 PR 会被拒绝。重大贡献可由维护者酌情例外。
- **PR 与 issue 中不得出现凭据。** `MUSIC_U` 等 cookie、手机号、密码、设备 ID 以及真实账号抓到的请求体都必须脱敏。

## 开发环境

Rust 1.95 或更新版本。`rust-toolchain.toml` 请求当前 stable 工具链，使用 rustup 时 `cargo` 会选用已安装的最新 stable。crate 使用 edition 2024。

锁定的依赖要求 Rust 1.95：`compio-driver 0.12.4` 使用了 1.95 才稳定的
[`std::cfg_select!`](https://doc.rust-lang.org/stable/std/macro.cfg_select.html)。edition 2024 本身只要求 1.85，不能代表依赖的真实 MSRV。

系统构建依赖（与 CI 在 `ubuntu-24.04` 上安装的相同，默认 `native-tls` feature 需要）：

```bash
# Debian / Ubuntu
sudo apt install -y pkg-config libssl-dev
```

```bash
# Arch Linux
sudo pacman -S --needed pkg-config openssl
```

常用命令：

```bash
cargo build
cargo test --locked --all-targets
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo run --example cloudsearch -- 海阔天空   # 需要网络
```

测试不得访问真实服务。请求构造、请求头、cookie 与超时请像 `src/request/tests.rs` 那样用本地 TCP 服务验证；真实服务的行为用临时程序验证，并把结果写进 PR 说明，不要提交该程序。

## 仓库结构

| 路径 | 内容 |
| --- | --- |
| `src/lib.rs` | crate 根：模块声明与公开导出（`ApiClient`、`ApiResponse`、`CryptoType`、`RequestOption`、`Query`、`NcmError`）。 |
| `src/request.rs` | `ApiClient`：按加密方式构造请求、请求头、客户端持有的 cookie jar 与 xeapi 状态、deadline 与回包解码。 |
| `src/crypto.rs` | weapi、eapi、linuxapi、xeapi 的加解密。 |
| `src/api/` | 一个上游模块一个文件，各自通过 `impl ApiClient` 增加方法；`mod.rs` 负责注册并定义 `Query`。 |
| `src/util/` | 上游常量（`config.rs`）、cookie 解析与会话 jar（`cookie.rs`）、设备标识与 IP 工具。 |
| `src/error.rs` | `NcmError` 与 crate 的 `Result` 别名。 |
| `examples/` | 可运行示例。 |
| `docs/API.md` | 接口文档（简体中文，沿用上游文档结构）。 |
| `.github/` | CI（`workflows/ci.yml`）、发版流程（`workflows/release.yml`）及其辅助脚本。 |
| 根目录文档 | `README.md` / `README_zh.md`（两种语言并行维护）、`CHANGELOG*.md`、`CONTRIBUTING*.md`、`CODE_OF_CONDUCT*.md`、`DISCLAIMER*.md`、`ACKNOWLEDGEMENTS*.md`、`LICENSE`（MIT）。 |

## 移植接口

新增与修改的接口以上游项目为准，CRIA 不自创接口。

1. 以 [NeteaseCloudMusicApi Enhanced](https://github.com/NeteaseCloudMusicApiEnhanced/api-enhanced) 的 `module/<name>.js` 为起点，Rust 文件与方法同名（`src/api/<name>.rs`、`ApiClient::<name>`）。
2. 保持上游语义：URL、加密方式、请求体字段、默认值、cookie 覆盖以及回包后处理。有意的差异要在注释里说明原因。
3. 像现有接口一样写上 `/// 对应 Node.js module/<name>.js` 文档行，并在 `src/api/mod.rs` 对应分组中注册模块。
4. 按相邻条目的格式在 `docs/API.md` 中补充文档。
5. PR 中附上所移植的上游文件与提交链接，并说明验证方式。

公共请求行为（`src/request.rs`、`src/crypto.rs`、`src/util/`）的改动会影响所有接口：请补测试，并保持 README 中的客户端约定准确。

## 有意不移植的上游模块

CRIA 有意不把以下 `api-enhanced` 模块作为独立的 `ApiClient` 方法公开：

- `decrypt`：上游服务端调试辅助接口；CRIA 改为提供本地 EAPI 解密实现。
- `register_checktoken_v2` / `register_checktoken_v3`：易盾 token 获取已内置在请求层。
- `register_xeapikey`：xeapi 公钥注册已内置在请求层。
- `register_neapikey`：需要上游独立的 `neapi` 加密与密钥轮换子系统，CRIA 尚未实现。
- `scrobble_v1`：需要上游 NCBL 加密格式，CRIA 尚未实现。
- `song_url_match` / `song_url_ncmget`：第三方解灰辅助接口，不属于 API 客户端边界。

这些是明确的范围边界，不是遗漏的模块。重新考虑其中任何一项时，需要同时提出设计方案与验证计划。

## 代码约定

- **显式导入。** 写明所用条目，不要新增 glob 导入。
- **可见性最小化。** 不属于公开 API 的条目保持私有或 `pub(crate)`；不要为了测试放宽可见性。
- **测试放在同级文件。** `foo.rs` 的测试放在 `foo/tests.rs`（以 `#[cfg(test)] mod tests;` 声明）；生产模块不内嵌测试模块或仅测试用的辅助代码。
- **公开 API 的改动属于破坏性变更。** 重命名、删除公开条目或修改签名，需要与维护者商定版本号。

## 格式与 CI

格式化使用 `rustfmt` 与仓库的 `rustfmt.toml`：`edition = "2024"`、`max_width = 100`、`use_field_init_shorthand = true`。

lint 使用带 `-D warnings` 的 `cargo clippy`；`clippy.toml` 把 `too-many-arguments-threshold` 放宽到 8、`type-complexity-threshold` 放宽到 300。

CI（`.github/workflows/ci.yml`）在指向 `main` / `develop` 的 PR、推送到 `develop` 时运行，并作为发版流程的门禁被调用。所有任务均在 `ubuntu-24.04` 上执行并安装上述系统依赖：

- `build-and-test`：在 stable 上以默认 features 与 `--no-default-features` 分别运行 `cargo check` 与 `cargo test`（`--locked --all-targets`）。
- `quality`：在 stable 上并装好 `rustfmt`、`clippy` 组件后运行 `cargo fmt --all -- --check`，`--all-features` 与 `--no-default-features` 两次带 `-D warnings` 的 clippy，以及带 `RUSTDOCFLAGS=-D warnings` 的 `cargo doc --no-deps`。
- `package`：运行 `cargo package --locked`，按 crates.io 实际收到的打包源码构建一次。

逐项对应的本地命令：

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

仓库提交 `Cargo.lock`，让 CI 与贡献者使用相同的依赖版本；依赖变更请在同一个 PR 中更新它。

## 更新记录

改变 crate 行为或公开 API 的 PR，需要在 `CHANGELOG.md` 与 `CHANGELOG_zh.md` 的 `## [Unreleased]` 下对应的 `新增` / `变更` / `修复` / `移除`（英文版为 `Added` / `Changed` / `Fixed` / `Removed`）小节各加一条。不影响 crate 使用者的改动（测试、CI、内部重构）无需记录。

## 发版

版本号、tag 与发布说明以 GitHub 为准；crates.io 只接收 CI 依据 tag 发布的内容。发版由维护者执行：

1. 在 `develop` 上提 PR：修改 `Cargo.toml` 的 `version`（运行 `cargo check` 让 `Cargo.lock` 同步），把两份更新记录中的 `## [Unreleased]` 改为 `## [X.Y.Z] - YYYY-MM-DD`，补一个新的空 `## [Unreleased]`，并更新文末的对比链接。
2. 通过 PR 把 `develop` 合并到 `main`。
3. 在合并后的 `main` 提交上打 tag 并推送：`git tag -a vX.Y.Z -m "vX.Y.Z" origin/main && git push origin vX.Y.Z`。预发布版本如 `0.2.0-rc.1` 使用 tag `v0.2.0-rc.1`。

推送 tag 会触发 `.github/workflows/release.yml`：

- `verify`：要求 tag 等于 `v` + `Cargo.toml` 版本、tag 指向的提交位于 `main` 上、两份更新记录都有该版本的非空小节，并记录 crates.io 上是否已有该版本。
- `gate`：在 tag 提交上运行完整的 CI 工作流。
- `semver checks`：用 `cargo-semver-checks` 与 crates.io 上的最新版本对比，版本号升级幅度不足以覆盖 API 变化时失败。
- `publish`：在 `crates-io` environment 中运行 `cargo publish --locked`，通过 crates.io Trusted Publishing 认证，仓库不保存 crates.io token。
- `github-release`：用更新记录中该版本的小节（先英文后中文）创建 GitHub Release，预发布版本标记为 pre-release。

发版失败后可直接重跑：crates.io 已有的版本不会重复发布，已存在的 GitHub Release 会保留。crates.io 上的版本永远不能覆盖，出错只能发新版本（必要时 `cargo yank`）。需要演练时，在某个分支上手动运行 Release 工作流（`workflow_dispatch`）：它执行相同的检查并运行 `cargo publish --dry-run`，不发布任何内容。
