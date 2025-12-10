# Rust SOCKS5 Tool

A lightweight SOCKS5 proxy tool written in Rust, designed for small binary size and multi-platform support.

## Features

- **SOCKS5 Protocol**: Supports CONNECT command and NO AUTH authentication.
- **Async I/O**: Built with `tokio` for high performance.
- **Small Binary**: Optimized release profile for minimal size.
- **Multi-Platform**: CI builds for Linux, macOS, and Windows.

## Getting Started

### Prerequisites

- Rust (cargo)

### Building

```bash
cargo build --release
```

The binary will be located in `target/release/`.

### Running

```bash
# Run on default port 1080
cargo run --release

# Run on custom port and address
cargo run --release -- --port 8080 --address 127.0.0.1
```

### Usage

```
Usage: rust-socks5-tool [OPTIONS]

Options:
  -p, --port <PORT>        Port to listen on [default: 1080]
  -a, --address <ADDRESS>  Address to listen on [default: 0.0.0.0]
  -h, --help               Print help
  -V, --version            Print version
```

## CI/CD

This project uses GitHub Actions for continuous integration. It automatically builds and tests the project on Ubuntu, macOS, and Windows.

