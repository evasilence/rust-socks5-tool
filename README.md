# Rust SOCKS5 工具

一个使用 Rust 编写的轻量级 SOCKS5 代理工具，专为小体积二进制文件和多平台支持而设计。

## 功能特性

- **SOCKS5 协议**: 支持 CONNECT 和 UDP ASSOCIATE 命令。
- **认证支持**: 支持无认证（NO AUTH）模式和用户名/密码认证（USER/PASS）。
- **异步 I/O**: 基于 `tokio` 构建，提供高性能并发处理。
- **体积小巧**: 经过优化的发布配置文件，并使用 UPX 压缩，确保二进制文件体积最小。
- **多平台/多架构**: 支持 Linux (x64/ARM64), macOS (Intel/Apple Silicon), Windows (x64/x86 兼容 XP/2003)。

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
./rust-socks5-tool

# 指定端口和监听地址
./rust-socks5-tool --port 8080 --address 127.0.0.1

# 开启用户名密码认证
./rust-socks5-tool -u myuser -w mypassword
```

### 使用说明

```text
Usage: rust-socks5-tool [OPTIONS]

Options:
  -p, --port <PORT>          监听端口 [default: 1080]
  -a, --address <ADDRESS>    监听地址 [default: 0.0.0.0]
  -u, --username <USERNAME>  认证用户名 (可选)
  -w, --password <PASSWORD>  认证密码 (可选)
  -h, --help                 Print help
  -V, --version              Print version
```

## 后台运行与服务化

### Linux (Systemd)

推荐使用 `systemd` 管理服务。

1. 创建服务文件 `/etc/systemd/system/rust-socks5.service`:

```ini
[Unit]
Description=Rust SOCKS5 Proxy Service
After=network.target

[Service]
Type=simple
User=nobody
ExecStart=/usr/local/bin/rust-socks5-tool --port 1080 --username myuser --password mypass
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

2. 启动并设置开机自启:

```bash
sudo systemctl daemon-reload
sudo systemctl start rust-socks5
sudo systemctl enable rust-socks5
```

### macOS (Launchd)

使用 `launchd` 进行管理。

1. 创建配置文件 `~/Library/LaunchAgents/com.user.rust-socks5.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.user.rust-socks5</string>
    <key>ProgramArguments</key>
    <array>
        <string>/path/to/rust-socks5-tool</string>
        <string>--port</string>
        <string>1080</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
```

2. 加载服务:

```bash
launchctl load ~/Library/LaunchAgents/com.user.rust-socks5.plist
```

### Windows

#### 方法 1: 使用 NSSM (推荐)

[NSSM](https://nssm.cc/) 是一个封装器，可以将普通 exe 注册为 Windows 服务。

1. 下载并解压 NSSM。
2. 以管理员身份运行 CMD:
   ```cmd
   nssm install RustSocks5
   ```
3. 在弹出的窗口中选择 `rust-socks5-tool.exe` 路径，并在 Arguments 中填入参数（如 `--port 1080`）。
4. 点击 "Install service"。
5. 启动服务: `nssm start RustSocks5`

#### 方法 2: PowerShell 后台运行

```powershell
Start-Process -FilePath ".\rust-socks5-tool.exe" -ArgumentList "--port 1080" -NoNewWindow -PassThru
```

### Docker 部署

本项目提供了 `Dockerfile`，你可以直接构建并运行 Docker 容器。

1. 构建镜像:
   ```bash
   docker build -t rust-socks5-tool .
   ```

2. 运行容器:
   ```bash
   docker run -d -p 1080:1080 --name socks5 rust-socks5-tool
   ```

   或者带参数运行:
   ```bash
   docker run -d -p 1080:1080 --name socks5 rust-socks5-tool rust-socks5-tool -u myuser -w mypass
   ```

## CI/CD 持续集成

本项目使用 GitHub Actions 进行持续集成。
- 自动在 Ubuntu, macOS, 和 Windows 上构建。
- 推送 `v*` 标签（如 `v0.1.0`）时自动发布 Release 并上传构建产物。

## 更新日志

### v0.1.6 (2025-12-10)
- **新特性**:
  - 支持用户名/密码认证 (RFC 1929)。
  - 支持 UDP ASSOCIATE 协议（基础握手）。
  - 新增多架构支持：Linux ARM64 (`aarch64`) 和 Windows 32位 (`i686`, 兼容 XP/2003)。
  - CLI 帮助信息全面中文化。
- **修复**:
  - 修复 Windows 构建脚本在 PowerShell 环境下的兼容性问题。

### v0.1.3
- **修复**: 修正 Windows 平台 UPX 压缩产物的上传路径问题。

### v0.1.2
- **优化**: 集成 UPX 压缩工具，显著减小 Linux 和 Windows 平台的二进制文件体积。

### v0.1.1
- **优化**: 引入 `tracing` 结构化日志系统。
- **优化**: 精简 `tokio` 依赖特性，进一步减小二进制体积。

### v0.1.0
- **初始发布**: 实现基础 SOCKS5 CONNECT 代理功能。
- **自动化**: 配置 GitHub Actions 实现多平台（Linux/macOS/Windows）自动构建与发布。

