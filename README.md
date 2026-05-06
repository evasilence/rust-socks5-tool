# Rust SOCKS5 Tool

[![Build Status](https://github.com/evasilence/rust-socks5-tool/actions/workflows/release.yml/badge.svg)](https://github.com/evasilence/rust-socks5-tool/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[English](./README_EN.md) | [中文](./README.md)

一个使用 Rust 编写的轻量级、高性能 SOCKS5 代理工具。支持 TCP/UDP、身份验证以及跨平台使用。

## 功能特性

- 🚀 **高性能**: 基于 Rust 和 Tokio 构建，采用异步 I/O。
- 🔐 **身份验证**: 支持可选的用户名/密码认证 (RFC 1929)。
- 🌐 **SOCKS5 支持**: 完整支持 CONNECT (TCP) 和 UDP ASSOCIATE。
- 🖥️ **跨平台**: 支持 Linux (x86_64/ARM64)、macOS (Intel/Apple Silicon) 和 Windows (x86/x64)。
- 📦 **静态二进制**: 无需依赖，简单的单文件部署。

## 安装

### 下载二进制文件

请从 [Releases 页面](https://github.com/evasilence/rust-socks5-tool/releases) 下载适合您平台的最新版本。

### 从源码构建

请确保您已安装 Rust 和 Cargo。

```bash
git clone https://github.com/evasilence/rust-socks5-tool.git
cd rust-socks5-tool
cargo build --release
```

编译后的二进制文件位于 `target/release/rust-socks5-tool`。

## 使用方法

使用默认设置运行服务器（监听 0.0.0.0:1080）：

```bash
./rust-socks5-tool
```

### 命令行参数

```text
用法: rust-socks5-tool [选项]

选项:
  -p, --port <PORT>          监听端口 [默认: 1080]
  -a, --address <ADDRESS>    监听地址 [默认: 0.0.0.0]
  -u, --username <USERNAME>  认证用户名 (可选，须与 --password 同时使用)
  -w, --password <PASSWORD>  认证密码 (可选，须与 --username 同时使用)
  -v, --debug                启用调试日志 (等同于 RUST_LOG=debug)
  -h, --help                 打印帮助信息
  -V, --version              打印版本信息
```

> **注意：** `--username` 与 `--password` 必须同时提供。
> 日志级别也可通过 `RUST_LOG` 环境变量控制（例如 `RUST_LOG=debug ./rust-socks5-tool`）。

### 示例

**1. 监听自定义端口：**

```bash
./rust-socks5-tool --port 8080
```

**2. 仅在本地监听：**

```bash
./rust-socks5-tool --address 127.0.0.1
```

**3. 启用用户名/密码认证：**

```bash
./rust-socks5-tool --username myuser --password mysecret
```

**4. 启用调试日志：**

```bash
./rust-socks5-tool --debug
# 或通过环境变量
RUST_LOG=debug ./rust-socks5-tool
```

## 客户端配置

您可以使用任何兼容 SOCKS5 的客户端进行连接。

**使用 curl 的示例：**

```bash
curl --socks5-hostname 127.0.0.1:1080 http://ifconfig.me
```

**使用 curl 的示例（带认证）：**

```bash
curl --socks5-hostname 127.0.0.1:1080 --proxy-user myuser:mysecret http://ifconfig.me
```

## 贡献

欢迎提交 Pull Request！

## 许可证

本项目采用 MIT 许可证。
