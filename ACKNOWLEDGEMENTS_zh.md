<p align="center">
	<a href="ACKNOWLEDGEMENTS.md">English</a>
	&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
	<a href="ACKNOWLEDGEMENTS_zh.md">简体中文</a>
</p>

# 致谢

没有下列项目就没有 CRIA。本文件向它们致谢，并说明 CRIA 由其衍生部分适用的许可。CRIA 自身代码以
[MIT 许可证](LICENSE) 发布。各上游许可证的完整原文保存在英文版 [ACKNOWLEDGEMENTS.md](ACKNOWLEDGEMENTS.md) 中，
以英文原文为准。

## NeteaseCloudMusicApi Enhanced

- 上游：https://github.com/NeteaseCloudMusicApiEnhanced/api-enhanced
- 许可：MIT，版权所有 (c) 2013-2022 Binaryify
- 关系：CRIA 是该 Node.js 项目的 Rust 移植。`src/api/` 下的各接口模块、`src/request.rs` 中的请求构造与 cookie
  处理、`src/crypto.rs` 中的加密方式以及 `src/util/config.rs` 中的常量，均对应上游 `module/` 与 `util/` 的实现；
  `docs/API.md` 改编自上游文档。

感谢维持这套接口可用的贡献者，以及逆向 weapi、eapi、xeapi 加密方式的各位。

## Binaryify/NeteaseCloudMusicApi

- 上游：https://github.com/Binaryify/NeteaseCloudMusicApi
- 关系：NeteaseCloudMusicApi Enhanced 所延续的原始 Node.js 网易云音乐 API 项目。

## ncm-api-rs

- 上游：https://github.com/imsyy/ncm-api-rs（现为 https://github.com/SPlayer-Dev/ncm-api-rs），作者 imsyy
- 许可：WTFPL 第 2 版
- 关系：CRIA 的代码基础来自这个 Rust 移植。它于 2026 年 3 月被 vendored 进 CNMPlayer，在那里持续维护（迁移到
  compio/cyper、加入 xeapi、会话归客户端所有等），随后拆分为 CRIA。WTFPL 允许以其他条款再分发；CRIA 以 MIT
  发布，并在英文版中保留原许可声明。

## CNMPlayer

- 仓库：https://github.com/professor-lee/CNMPlayer
- 关系：CRIA 原是 CNMPlayer 的网络层。本仓库历史由 CNMPlayer 的 `ncm-api-rs/` 目录拆分而来，在那里产生的提交及其作者均得以保留。

## Rust 生态

CRIA 使用 [compio](https://github.com/compio-rs/compio) 与 [cyper](https://github.com/compio-rs/cyper) 作为运行时与 HTTP
客户端，使用 [RustCrypto](https://github.com/RustCrypto) 系列 crate 实现 AES、RSA、HMAC、SHA-2 与 MD5，使用
[x25519-dalek](https://github.com/dalek-cryptography/curve25519-dalek) 完成 xeapi 密钥交换，另有 serde、chrono、flate2、
regex-lite 等 `Cargo.toml` 中列出的依赖。这些 crate 经 Cargo 引入，各自保留其许可证。

## 说明

本文件保留上述声明，并非每个 Cargo 依赖的完整许可清单，也不构成法律意见。
