<h1 align="center">CRIA</h1>

<p align="center">
	<a href="README.md">English</a>
	&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
	<a href="README_zh.md">简体中文</a>
</p>

<p align="center" style="color:gray;">
	Cloud-music Rust Integration API —— 网易云音乐 API 的 Rust 原生客户端。
</p>

<p align="center">
    <img src="https://img.shields.io/badge/Language-Rust-orange?logo=rust&logoColor=white" alt="Rust">
    <img src="https://img.shields.io/badge/Runtime-compio-informational" alt="compio">
    <img src="https://img.shields.io/badge/License-MIT-blue?logo=opensourceinitiative&logoColor=white" alt="License">
    <a href="https://github.com/professor-lee/cria/actions/workflows/ci.yml"><img src="https://github.com/professor-lee/cria/actions/workflows/ci.yml/badge.svg?branch=develop" alt="CI"></a>
</p>

## 项目简介

CRIA 是社区 Node.js 项目 [NeteaseCloudMusicApi Enhanced](https://github.com/NeteaseCloudMusicApiEnhanced/api-enhanced)
的 Rust 移植。应用不必再起一个 Node.js HTTP 服务，直接链接 CRIA 调用接口：

- `ApiClient` 上约 370 个接口方法，一个上游模块对应一个文件，参数统一经 `Query` 传入；
- 支持 weapi / eapi / linuxapi / xeapi 加密与明文 api 请求，包括 xeapi 的公钥注册与会话握手；
- 会话归客户端所有：自动合并响应的 `Set-Cookie`，设备 ID 由调用方持久化，每个请求自己设置 User-Agent 与 Referer；
- 运行在 [compio](https://github.com/compio-rs/compio) 运行时上，经注入的 [`cyper::Client`](https://github.com/compio-rs/cyper)
  发请求，代理与连接复用由调用方掌控。

CRIA 原是 [CNMPlayer](https://github.com/professor-lee/CNMPlayer) 的网络层，保留提交历史从该仓库拆分而来。
CNMPlayer 是它的第一个使用方，对自己用到的接口做过真实服务验证；其余接口是对上游的忠实移植，未逐一实测。

> CRIA 是非官方项目，与网易云音乐无任何关联。使用前请阅读 [DISCLAIMER_zh.md](DISCLAIMER_zh.md)。

## 安装

crates.io 包名为 **`cria-api`**，代码中的导入名为 **`cria`**。首次发布到 crates.io 之前，请直接依赖本仓库并锁定提交：

```toml
[dependencies]
cria-api = { git = "https://github.com/professor-lee/cria", rev = "<commit>" }
cyper = "0.9"
compio = { version = "0.19", features = ["macros"] }
```

发布到 crates.io 后改为 `cria-api = "0.1"`。

构建需要 Rust 1.95 或更新版本（compio 依赖链用到 `std::cfg_select!`）；默认的 `native-tls` feature 还需要系统
OpenSSL 开发包（Debian/Ubuntu 为 `libssl-dev`，Arch 为 `openssl`）。

## 快速开始

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

完整示例见 [`examples/cloudsearch.rs`](examples/cloudsearch.rs)：`cargo run --example cloudsearch -- 海阔天空`。

所有接口返回 `Result<ApiResponse, NcmError>`。`ApiResponse` 包含业务 `status`、原始 JSON `body` 与响应的
`Set-Cookie`；响应不映射为类型化模型，按上游文档描述从 `body` 读取字段。接口说明见 [`docs/API.md`](docs/API.md)。

## 客户端约定

- **运行时。** `ApiClient` 包装的 `cyper::Client` 是 `!Send`，请在创建它的 compio 运行时线程上使用。克隆开销很小，且共享同一会话。
- **网络配置。** 代理、DNS 与连接池在注入的 `cyper::Client` 上配置，`Query` 与请求选项不携带代理设置。请求头由库设置，不继承 client 的默认请求头。
- **会话。** 未通过 `Query` 指定 cookie 的请求使用客户端会话，并把响应 `Set-Cookie` 合并回去（`Max-Age<=0` 或已过期的
  `Expires` 视为删除）。`set_cookie` / `clear_cookie` 整体替换或清空会话，此前发出的请求不会把旧 cookie 写回。
  用 `cookie()` 取出会话持久化，用 `cookie_value("MUSIC_U")` 判断是否登录。经 `Query` 传入的 cookie 只作用于该次请求。
- **设备身份。** `ApiClient::new` 会生成随机设备 ID。请持久化 `device_id()` 并在下次启动时用 `set_device_id` 恢复，
  让服务端始终看到同一台设备。xeapi 公钥与会话按客户端保存。
- **超时。** 连接、响应头与完整 body 共用一个 deadline：`ApiClient::DEFAULT_TIMEOUT`（30 秒），可用 `set_timeout`
  调整，超时返回 `NcmError::Timeout`。

## Cargo features

| Feature | 默认 | 作用 |
| --- | --- | --- |
| `native-tls` | 开 | 通过系统 TLS 实现访问 HTTPS（`cyper/native-tls`）。 |
| `gzip` | 开 | 接受并解码 gzip 压缩的 API 回包。 |
| `brotli` | 开 | 接受并解码 brotli 压缩的 API 回包。 |

使用 `--no-default-features` 仍能编译，但除非构建中另有 crate 为 `cyper` 开启 TLS 后端，否则无法访问 HTTPS 接口。

## 项目状态

- 版本 `0.1.0`，尚未发布到 crates.io。
- API 与上游模块集合对应，新接口随上游变化补充。
- 集成测试使用本地 TCP 服务，不会访问真实服务。

## 参与贡献

分支规则、开发命令、CI 与接口移植方式见 [CONTRIBUTING_zh.md](CONTRIBUTING_zh.md)，社区行为准则见
[CODE_OF_CONDUCT_zh.md](CODE_OF_CONDUCT_zh.md)。

## 致谢

CRIA 建立在 NeteaseCloudMusicApi 系列项目与早期 Rust 移植 `ncm-api-rs` 的工作之上，
[ACKNOWLEDGEMENTS_zh.md](ACKNOWLEDGEMENTS_zh.md) 列出了它们及其许可声明。

## 许可证

[MIT](LICENSE)。通过本库访问的内容与服务不受该许可约束，详见 [DISCLAIMER_zh.md](DISCLAIMER_zh.md)。
