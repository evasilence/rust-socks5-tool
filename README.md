# Rust SOCKS5 工具

一个使用 Rust 编写的轻量级 SOCKS5 代理工具，专为小体积二进制文件和多平台支持而设计。

## 功能特性

- **SOCKS5 协议**: 支持 CONNECT 命令和无认证（NO AUTH）模式。
- **异步 I/O**: 基于 `tokio` 构建，提供高性能并发处理。
- **体积小巧**: 经过优化的发布配置文件，确保二进制文件体积最小。
- **多平台支持**: 通过 CI 自动构建 Linux, macOS 和 Windows 版本。

## 快速开始

### 下载安装

你可以直接从 [Releases](https://github.com/evasilence/rust-socks5-tool/releases) 页面下载对应平台的预编译二进制文件。

### 源码编译

如果你需要从源码编译，请确保已安装 Rust 环境。

```bash
cargo build --release
```

编译后的二进制文件位于 `target/release/` 目录下。

### 运行

```bash
# 在默认端口 1080 上运行
cargo run --release

# 指定端口和监听地址运行
cargo run --release -- --port 8080 --address 127.0.0.1
```

或者直接运行编译好的二进制文件：

```bash
./rust-socks5-tool --port 1080
```

### 使用说明

```text
Usage: rust-socks5-tool [OPTIONS]

Options:
  -p, --port <PORT>        监听端口 [默认: 1080]
  -a, --address <ADDRESS>  监听地址 [默认: 0.0.0.0]
  -h, --help               打印帮助信息
  -V, --version            打印版本信息
```

## CI/CD 持续集成

本项目使用 GitHub Actions 进行持续集成。
- 自动在 Ubuntu, macOS, 和 Windows 上构建。
- 推送 `v*` 标签（如 `v0.1.0`）时自动发布 Release 并上传构建产物。

