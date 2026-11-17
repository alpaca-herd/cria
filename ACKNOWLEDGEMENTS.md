<p align="center">
	<a href="ACKNOWLEDGEMENTS.md">English</a>
	&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
	<a href="ACKNOWLEDGEMENTS_zh.md">简体中文</a>
</p>

# Acknowledgements

CRIA exists because of the projects below. This file credits them and preserves the license notices that
apply to the material CRIA derives from them. CRIA's own code is licensed under the [MIT License](LICENSE).

## NeteaseCloudMusicApi Enhanced

- Upstream: https://github.com/NeteaseCloudMusicApiEnhanced/api-enhanced
- License: MIT, copyright (c) 2013-2022 Binaryify
- Relationship: CRIA is a Rust port of this Node.js project. The endpoint modules under `src/api/`, the
  request construction and cookie handling in `src/request.rs`, the encryption schemes in `src/crypto.rs`
  and the constants in `src/util/config.rs` follow the upstream `module/` and `util/` implementations.
  `docs/API.md` is adapted from the upstream documentation.

Thanks to the contributors who keep the API working and to everyone who reverse-engineered the weapi,
eapi and xeapi schemes.

### Upstream license text

```text
The MIT License (MIT)

Copyright (c) 2013-2022 Binaryify

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
```

## Binaryify/NeteaseCloudMusicApi

- Upstream: https://github.com/Binaryify/NeteaseCloudMusicApi
- Relationship: the original Node.js NetEase Cloud Music API that NeteaseCloudMusicApi Enhanced continues.

## ncm-api-rs

- Upstream: https://github.com/imsyy/ncm-api-rs (now https://github.com/SPlayer-Dev/ncm-api-rs), by imsyy
- License: WTFPL, Version 2
- Relationship: CRIA's code base started from this Rust port. It was vendored into CNMPlayer in March 2026,
  maintained there (compio/cyper migration, xeapi, client-owned sessions and other changes) and split out as
  CRIA. The WTFPL permits redistribution under other terms; CRIA is distributed under MIT and keeps the
  original notice below.

### Upstream license text

```text
            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
                    Version 2, December 2004

 Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>

 Everyone is permitted to copy and distribute verbatim or modified
 copies of this license document, and changing it is allowed as long
 as the name is changed.

            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

  0. You just DO WHAT THE FUCK YOU WANT TO.
```

## CNMPlayer

- Repository: https://github.com/professor-lee/CNMPlayer
- Relationship: CRIA was developed as CNMPlayer's networking layer. This repository's history was split from
  CNMPlayer's `ncm-api-rs/` directory, so the commits made there, and their authors, are preserved.

## Rust ecosystem

CRIA relies on [compio](https://github.com/compio-rs/compio) and [cyper](https://github.com/compio-rs/cyper)
for the runtime and HTTP client, the [RustCrypto](https://github.com/RustCrypto) crates for AES, RSA, HMAC,
SHA-2 and MD5, [x25519-dalek](https://github.com/dalek-cryptography/curve25519-dalek) for the xeapi key
exchange, and serde, chrono, flate2, regex-lite and the other dependencies listed in `Cargo.toml`. Those
crates are used through Cargo and keep their own licenses.

## Notes

This file preserves the notices listed above; it is not an exhaustive license inventory for every Cargo
dependency and is not legal advice.
