# Rust SOCKS5 Tool

[![Build Status](https://github.com/evasilence/rust-socks5-tool/actions/workflows/release.yml/badge.svg)](https://github.com/evasilence/rust-socks5-tool/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-v1.0.1-blue.svg)](https://github.com/evasilence/rust-socks5-tool/releases/tag/v1.0.1)

[English](./README_EN.md) | [中文](./README.md)

A lightweight, high-performance SOCKS5 proxy tool written in Rust. Supports TCP/UDP, authentication, and cross-platform usage.

## Features

- 🚀 **High Performance**: Built with Rust and Tokio for asynchronous I/O.
- 🔐 **Authentication**: Optional username/password authentication support (RFC 1929).
- 🌐 **SOCKS5 Support**: Full support for CONNECT (TCP) and UDP ASSOCIATE.
- 🖥️ **Cross-Platform**: Runs on Linux (x86_64/ARM64), macOS (Intel/Apple Silicon), and Windows (x86/x64).
- 📦 **Static Binary**: No dependencies required, simple single-file deployment.
- ⚡ **Low Latency**: TCP_NODELAY on target connections reduces latency for interactive protocols (SSH, HTTP).

## Installation

### Download Binaries

Download the latest release for your platform from the [Releases Page](https://github.com/evasilence/rust-socks5-tool/releases).

### Build from Source

Ensure you have Rust and Cargo installed.

```bash
git clone https://github.com/evasilence/rust-socks5-tool.git
cd rust-socks5-tool
cargo build --release
```

The binary will be located in `target/release/rust-socks5-tool`.

## Usage

Run the server with default settings (Listen on 0.0.0.0:1080):

```bash
./rust-socks5-tool
```

### Command Line Arguments

```text
Usage: rust-socks5-tool [OPTIONS]

Options:
  -p, --port <PORT>          Listening port [default: 1080]
  -a, --address <ADDRESS>    Listening address [default: 0.0.0.0]
  -u, --username <USERNAME>  Authentication username (optional, must be paired with --password)
  -w, --password <PASSWORD>  Authentication password (optional, must be paired with --username)
  -v, --debug                Enable debug logging (equivalent to RUST_LOG=debug)
  -h, --help                 Print help
  -V, --version              Print version
```

> **Note:** `--username` and `--password` must always be provided together.
> Log verbosity can also be controlled via the `RUST_LOG` environment variable
> (e.g. `RUST_LOG=debug ./rust-socks5-tool`).

### Examples

**1. Listen on a custom port:**

```bash
./rust-socks5-tool --port 8080
```

**2. Listen on localhost only:**

```bash
./rust-socks5-tool --address 127.0.0.1
```

**3. Enable UserName/Password Authentication:**

```bash
./rust-socks5-tool --username myuser --password mysecret
```

**4. Enable debug logging:**

```bash
./rust-socks5-tool --debug
# or via environment variable
RUST_LOG=debug ./rust-socks5-tool
```

## Client Configuration

You can use any SOCKS5 compatible client to connect.

**Example with curl:**

```bash
curl --socks5-hostname 127.0.0.1:1080 http://ifconfig.me
```

**Example with curl (Authenticated):**

```bash
curl --socks5-hostname 127.0.0.1:1080 --proxy-user myuser:mysecret http://ifconfig.me
```

## Changelog

See [CHANGELOG.md](./CHANGELOG.md) for the full version history.

### v1.0.1 Highlights

- **TCP_NODELAY**: Set on every target connection to reduce latency for interactive protocols.
- **Per-address connect timeout**: Each resolved address is bounded by an independent 5 s deadline, preventing a single unresponsive host from blocking the entire handshake.
- **SOCKS5 error code fix**: `PermissionDenied` now correctly returns reply code `0x02` ("Connection not allowed") instead of `0x04`.
- **UDP IPv4 preference**: Domain resolution in the UDP relay path now prefers IPv4, matching TCP relay behaviour.
- **UDP fragmentation debug logs**: Dropped malformed/fragmented UDP packets now emit `debug!` messages for easier diagnostics.
- **TCP relay observability**: `ConnectionReset` / `BrokenPipe` relay events now emit a `debug!` log.

## Contributing

Contributions are welcome! Please submit a Pull Request.

## License

This project is licensed under the MIT License.


## Installation

### Download Binaries

Download the latest release for your platform from the [Releases Page](https://github.com/evasilence/rust-socks5-tool/releases).

### Build from Source

Ensure you have Rust and Cargo installed.

```bash
git clone https://github.com/evasilence/rust-socks5-tool.git
cd rust-socks5-tool
cargo build --release
```

The binary will be located in `target/release/rust-socks5-tool`.

## Usage

Run the server with default settings (Listen on 0.0.0.0:1080):

```bash
./rust-socks5-tool
```

### Command Line Arguments

```text
Usage: rust-socks5-tool [OPTIONS]

Options:
  -p, --port <PORT>          Listening port [default: 1080]
  -a, --address <ADDRESS>    Listening address [default: 0.0.0.0]
  -u, --username <USERNAME>  Authentication username (optional, must be paired with --password)
  -w, --password <PASSWORD>  Authentication password (optional, must be paired with --username)
  -v, --debug                Enable debug logging (equivalent to RUST_LOG=debug)
  -h, --help                 Print help
  -V, --version              Print version
```

> **Note:** `--username` and `--password` must always be provided together.
> Log verbosity can also be controlled via the `RUST_LOG` environment variable
> (e.g. `RUST_LOG=debug ./rust-socks5-tool`).

### Examples

**1. Listen on a custom port:**

```bash
./rust-socks5-tool --port 8080
```

**2. Listen on localhost only:**

```bash
./rust-socks5-tool --address 127.0.0.1
```

**3. Enable UserName/Password Authentication:**

```bash
./rust-socks5-tool --username myuser --password mysecret
```

**4. Enable debug logging:**

```bash
./rust-socks5-tool --debug
# or via environment variable
RUST_LOG=debug ./rust-socks5-tool
```

## Client Configuration

You can use any SOCKS5 compatible client to connect.

**Example with curl:**

```bash
curl --socks5-hostname 127.0.0.1:1080 http://ifconfig.me
```

**Example with curl (Authenticated):**

```bash
curl --socks5-hostname 127.0.0.1:1080 --proxy-user myuser:mysecret http://ifconfig.me
```

## Contributing

Contributions are welcome! Please submit a Pull Request.

## License

This project is licensed under the MIT License.

