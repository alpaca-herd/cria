//! CRIA (Cloud-music Rust Integration API)：网易云音乐 API 的 Rust 原生客户端。
//!
//! 移植自 [NeteaseCloudMusicApi Enhanced](https://github.com/NeteaseCloudMusicApiEnhanced/api-enhanced)
//! 的 Node.js 实现，运行在 compio 异步运行时上；支持 weapi / eapi / linuxapi / xeapi
//! 加密方式与明文 api 请求。接口说明见仓库 `docs/API.md`。

#![deny(unsafe_code)]

pub mod api;
pub mod crypto;
pub mod error;
pub mod request;
pub mod util;

pub use api::Query;
pub use error::NcmError;
pub use request::{ApiClient, ApiResponse, CryptoType, RequestOption};
