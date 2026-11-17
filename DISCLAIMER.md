<p align="center">
	<a href="DISCLAIMER.md">English</a>
	&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
	<a href="DISCLAIMER_zh.md">简体中文</a>
</p>

# Disclaimer

Last updated: **2026-11-17**

> **Read before use:** CRIA is an unofficial, third-party open-source library. Its software license does
> **not** license music or any other platform content, and it does not authorize access to the service on
> anyone's behalf. Whoever builds on CRIA is responsible for having a lawful basis for what their software does.
> The software is provided **"as is"**; warranty exclusions and liability limits apply only to the extent
> permitted by applicable law.

## 1. Scope

This notice covers CRIA's source code, published crates, documentation and examples. "Content" means
material obtained from external services, including recordings, compositions, cover art, lyrics, images,
artist, album and playlist information, and metadata. "Integrators" means developers and operators of
software that uses CRIA; "maintainers" means CRIA's authors and contributors.

This notice is not a substitute for the [LICENSE](LICENSE), a content license, the external service's
applicable terms or mandatory law, and it does not create a separate agreement merely because someone
visits the repository, downloads the crate or reads this page.

## 2. Unofficial status

CRIA is not an official NetEase Cloud Music product or SDK. It has no affiliation, authorization,
sponsorship, partnership or endorsement from NetEase Cloud Music or its affiliates. The endpoints,
encryption schemes and request formats it implements were derived by the community from public client
behavior and from [NeteaseCloudMusicApi Enhanced](https://github.com/NeteaseCloudMusicApiEnhanced/api-enhanced);
they are not a published or guaranteed interface.

Names and trademarks mentioned in the code or documentation identify services or projects and remain
subject to their owners' rights.

## 3. Interfaces can change or stop working

The service can change or withdraw endpoints, parameters, encryption, risk controls or account rules at
any time and without notice. An endpoint that works today may fail, return different data or trigger
verification later. Apart from the endpoints CNMPlayer uses, CRIA's endpoints are faithful ports of the
upstream project that the maintainers have not individually verified against the live service.

The project promises no uninterrupted operation, compatibility, response time or continued maintenance.
Do not build safety-critical, business-critical or data-retention-critical systems on it.

## 4. Content rights

Rights in external content remain with their respective rights holders. CRIA's MIT license covers CRIA's
code only. Requesting, playing, caching, downloading or storing content through software built with CRIA
does not transfer ownership or grant rights to copy, distribute, publicly perform or commercially exploit
it. Membership, a purchased track or a returned media URL does not by itself establish permission for
third-party-client access, independent downloads or redistribution.

## 5. Integrators' responsibilities

Integrators decide how CRIA is used and are responsible for complying with applicable law and the service's
legally applicable terms. In particular, do not use CRIA, without the necessary authorization or another
lawful basis, to:

- redistribute recordings, artwork or lyrics, or operate a substitute music service, public download
  endpoint, relay or content library;
- unlawfully circumvent payment, DRM, account entitlements, geographic restrictions, authentication or
  anti-abuse controls;
- perform bulk extraction, mirroring or dataset building where the necessary rights are absent;
- use other people's accounts, stolen credentials or session cookies, or make excessive automated requests
  that evade rate limits and account restrictions.

Request options such as `real_ip` and `random_cn_ip` set forwarding headers on outgoing requests. They are
ports of upstream options, not a promise that any restriction may lawfully be bypassed.

## 6. Accounts and credentials

Third-party-client access may be restricted by platform terms or risk controls; consequences can include
additional verification, invalidated sessions, rate limiting or account suspension. The project does not
guarantee that any account remains usable.

CRIA keeps the session cookie, device ID and xeapi session in memory only. Persisting them, protecting
the stored files and deleting them is the integrator's responsibility. Treat cookies such as `MUSIC_U` as
credentials: do not log, publish or share them, and do not include them in issue reports.

## 7. Network requests and data

Requests go to NetEase Cloud Music endpoints (including `music.163.com`, `interface.music.163.com`,
`interfacepc.music.163.com`, `interface3.music.163.com` and `clientlog.music.163.com`), to NetEase's anti-abuse service
(`ac.dun.163yun.com`) when an endpoint requires a token, and to upload or media endpoints used by specific
endpoints. They can disclose the caller's IP address, cookies, device identifiers, requested content and other
request metadata to those services and to network intermediaries. Those services apply their own terms and
privacy practices; the maintainers do not control them.

## 8. Warranty and liability

**To the extent permitted by applicable law, CRIA is provided "AS IS", without warranty of any kind.** The
controlling terms are in the [LICENSE](LICENSE). Nothing in this notice excludes or limits liability or
rights that applicable law does not allow to be excluded or limited.

## 9. Reports

If you believe material in this repository infringes your rights, report it through
[GitHub Issues](https://github.com/alpaca-herd/cria/issues). For reports containing personal information,
confidential evidence or an exploitable security issue, contact the maintainer at
[mail@professorlee.work](mailto:mail@professorlee.work) instead of disclosing it publicly. Do not attach
passwords, cookies or session files.

## 10. Languages

The [English](DISCLAIMER.md) and [Simplified Chinese](DISCLAIMER_zh.md) versions are intended to have the same
meaning. This document is general project information, **not legal advice**.
