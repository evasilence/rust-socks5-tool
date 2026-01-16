# Rust SOCKS5 Tool

[![Build Status](https://github.com/evasilence/rust-socks5-tool/actions/workflows/release.yml/badge.svg)](https://github.com/evasilence/rust-socks5-tool/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[English](./README_EN.md) | [中文](./README.md)

A lightweight, high-performance SOCKS5 proxy tool written in Rust. Supports TCP/UDP, authentication, and cross-platform usage.

## Features

- 🚀 **High Performance**: Built with Rust and Tokio for asynchronous I/O.
- 🔐 **Authentication**: Optional username/password authentication support (RFC 1929).
- 🌐 **SOCKS5 Support**: Full support for CONNECT (TCP) and UDP ASSOCIATE.
- 🖥️ **Cross-Platform**: Runs on Linux (x86_64/ARM64), macOS (Intel/Apple Silicon), and Windows (x86/x64).
- 📦 **Static Binary**: No dependencies required, simple single-file deployment.

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
  -u, --username <USERNAME>  Authentication username (optional)
  -w, --password <PASSWORD>  Authentication password (optional)
  -h, --help                 Print help
  -V, --version              Print version
```

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

