# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.1] - 2026-05-06

### Added
- **TCP_NODELAY** on target connections — disables Nagle's algorithm immediately after each TCP connect, reducing latency for interactive protocols (SSH, HTTP/1.1) tunnelled through the proxy.
- **Per-address connect timeout** (`CONNECT_TIMEOUT = 5 s`) — each resolved address is now bounded by its own deadline, preventing a single unresponsive address from stalling the entire handshake timeout.
- **UDP packet drop diagnostics** — silent drops of malformed/fragmented UDP packets are now surfaced as `debug!` log messages (too-short, non-zero RSV, `FRAG ≠ 0`), making client compatibility issues easier to diagnose.
- **TCP relay peer-close observability** — `ConnectionReset` / `BrokenPipe` relay events now emit a `debug!` log instead of being silently discarded.

### Changed
- **SOCKS5 error code correction** — `PermissionDenied` (EACCES/EPERM) now maps to reply code `0x02` ("Connection not allowed by ruleset") instead of incorrectly returning `0x04` ("Host unreachable").
- **UDP domain resolution IPv4 preference** — resolved addresses in the UDP relay path are now sorted IPv4-first before selection, matching existing TCP relay behaviour and avoiding IPv6 connectivity issues.
- **Simplified IPv4 preference sort** — replaced verbose `sort_by` / `Ordering` match with a concise `sort_by_key(|a| u8::from(a.is_ipv6()))` in `connect_to_target`.

---

## [1.0.0] - 2026-05-05

### Added
- Initial release of `rust-socks5-tool`.
- Lightweight SOCKS5 proxy server built on Rust + Tokio (async I/O).
- Full SOCKS5 **CONNECT** (TCP relay) support.
- Full SOCKS5 **UDP ASSOCIATE** support with in-place header rewriting.
- Optional **username/password authentication** (RFC 1929).
- TCP keepalive on accepted client connections (idle 60 s, interval 10 s).
- Graceful shutdown on `Ctrl-C` / SIGINT.
- SOCKS5 error reply codes mapped from `std::io::ErrorKind`.
- Structured logging via `tracing` / `tracing-subscriber` with `RUST_LOG` and `--debug` support.
- Cross-platform static binary distribution (Linux x86_64/ARM64, macOS Intel/Apple Silicon, Windows x86/x64).
- Optimised release profile: `opt-level = "z"`, LTO, single codegen unit, `panic = abort`, binary stripping.
