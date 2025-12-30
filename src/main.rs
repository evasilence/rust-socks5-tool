use anyhow::{Context, Result};
use clap::Parser;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket as TokioUdpSocket};
use tokio::time::timeout;
use tracing::{error, info, warn};

#[derive(Parser, Debug)]
#[command(author, version, about = "一个轻量级的 SOCKS5 代理工具", long_about = None)]
struct Args {
    /// 监听端口
    #[arg(short, long, default_value_t = 1080)]
    port: u16,

    /// 监听地址
    #[arg(short, long, default_value = "0.0.0.0")]
    address: String,

    /// 认证用户名 (可选)
    #[arg(short = 'u', long)]
    username: Option<String>,

    /// 认证密码 (可选)
    #[arg(short = 'w', long)]
    password: Option<String>,
}

const TIMEOUT_DURATION: Duration = Duration::from_secs(10);

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let args = Arc::new(Args::parse());
    let bind_addr = format!("{}:{}", args.address, args.port);

    let listener = TcpListener::bind(&bind_addr)
        .await
        .context(format!("Failed to bind to {}", bind_addr))?;

    info!("SOCKS5 server listening on {}", bind_addr);
    if args.username.is_some() {
        info!("Authentication enabled");
    }

    loop {
        tokio::select! {
            accept_result = listener.accept() => {
                match accept_result {
                    Ok((socket, addr)) => {
                        info!("Accepted connection from {}", addr);

                        // Set TCP Keepalive to prevent dead connections
                        let sock_ref = socket2::SockRef::from(&socket);
                        let mut ka = socket2::TcpKeepalive::new();
                        ka = ka.with_time(Duration::from_secs(60));
                        ka = ka.with_interval(Duration::from_secs(10));
                        
                        if let Err(e) = sock_ref.set_tcp_keepalive(&ka) {
                             warn!("Failed to set TCP keepalive for {}: {}", addr, e);
                        }

                        let args = args.clone();
                        tokio::spawn(async move {
                            if let Err(e) = handle_client(socket, args).await {
                                let msg = e.to_string();
                                // Reduce log level for common scanner/bot errors
                                if msg.contains("Authentication failed") 
                                    || msg.contains("early eof") 
                                    || msg.contains("unexpected end of file")
                                    || msg.contains("Handshake/Connection timeout") 
                                    || msg.contains("No supported authentication methods") {
                                    warn!("Client warning {}: {}", addr, msg);
                                } else {
                                    error!("Error handling client {}: {}", addr, e);
                                }
                            }
                        });
                    }
                    Err(e) => {
                        error!("Failed to accept connection: {}", e);
                    }
                }
            }
            _ = tokio::signal::ctrl_c() => {
                info!("Received shutdown signal, stopping server...");
                break;
            }
        }
    }

    Ok(())
}

async fn handle_client(mut client_stream: TcpStream, args: Arc<Args>) -> Result<()> {
    let client_addr = client_stream.peer_addr()?;
    // Wrap the entire handshake and request process in a timeout
    let (mut client_stream, target_stream_opt, udp_associate) = timeout(TIMEOUT_DURATION, async {
        // 1. Handshake
        let mut buf = [0u8; 2];
        client_stream.read_exact(&mut buf).await?;

        let ver = buf[0];
        let nmethods = buf[1];

        if ver != 0x05 {
            return Err(anyhow::anyhow!("Unsupported SOCKS version: {}", ver));
        }

        let mut methods = vec![0u8; nmethods as usize];
        client_stream.read_exact(&mut methods).await?;

        // Authentication Logic
        if let (Some(username), Some(password)) = (&args.username, &args.password) {
            if !methods.contains(&0x02) {
                client_stream.write_all(&[0x05, 0xFF]).await?;
                return Err(anyhow::anyhow!("Client does not support USERNAME/PASSWORD auth"));
            }

            // Select USERNAME/PASSWORD auth (0x02)
            client_stream.write_all(&[0x05, 0x02]).await?;

            // Auth Request
            let mut auth_ver = [0u8; 1];
            client_stream.read_exact(&mut auth_ver).await?;
            if auth_ver[0] != 0x01 {
                return Err(anyhow::anyhow!("Unsupported auth version: {}", auth_ver[0]));
            }

            let mut ulen = [0u8; 1];
            client_stream.read_exact(&mut ulen).await?;
            let mut uname = vec![0u8; ulen[0] as usize];
            client_stream.read_exact(&mut uname).await?;

            let mut plen = [0u8; 1];
            client_stream.read_exact(&mut plen).await?;
            let mut passwd = vec![0u8; plen[0] as usize];
            client_stream.read_exact(&mut passwd).await?;

            let client_username = String::from_utf8_lossy(&uname);
            let client_password = String::from_utf8_lossy(&passwd);

            if client_username != *username || client_password != *password {
                client_stream.write_all(&[0x01, 0x01]).await?; // Auth failed
                return Err(anyhow::anyhow!("Authentication failed"));
            }

            client_stream.write_all(&[0x01, 0x00]).await?; // Auth success
        } else {
            // No Auth
            if !methods.contains(&0x00) {
                client_stream.write_all(&[0x05, 0xFF]).await?;
                return Err(anyhow::anyhow!("No supported authentication methods"));
            }
            client_stream.write_all(&[0x05, 0x00]).await?;
        }

        // 2. Request
        let mut head = [0u8; 4];
        client_stream.read_exact(&mut head).await?;

        let ver = head[0];
        let cmd = head[1];
        let _rsv = head[2];
        let atyp = head[3];

        if ver != 0x05 {
            return Err(anyhow::anyhow!("Unsupported SOCKS version in request: {}", ver));
        }

        let target_addr_str = match atyp {
            0x01 => {
                // IPv4
                let mut addr_buf = [0u8; 4];
                client_stream.read_exact(&mut addr_buf).await?;
                let mut port_buf = [0u8; 2];
                client_stream.read_exact(&mut port_buf).await?;
                let port = u16::from_be_bytes(port_buf);
                format!("{}.{}.{}.{}:{}", addr_buf[0], addr_buf[1], addr_buf[2], addr_buf[3], port)
            }
            0x03 => {
                // Domain name
                let mut len_buf = [0u8; 1];
                client_stream.read_exact(&mut len_buf).await?;
                let len = len_buf[0] as usize;
                let mut domain_buf = vec![0u8; len];
                client_stream.read_exact(&mut domain_buf).await?;
                let domain = String::from_utf8_lossy(&domain_buf);
                let mut port_buf = [0u8; 2];
                client_stream.read_exact(&mut port_buf).await?;
                let port = u16::from_be_bytes(port_buf);
                format!("{}:{}", domain, port)
            }
            0x04 => {
                // IPv6
                let mut addr_buf = [0u8; 16];
                client_stream.read_exact(&mut addr_buf).await?;
                let mut port_buf = [0u8; 2];
                client_stream.read_exact(&mut port_buf).await?;
                let port = u16::from_be_bytes(port_buf);
                let addr = std::net::Ipv6Addr::from(addr_buf);
                format!("[{}]:{}", addr, port)
            }
            _ => {
                reply_error(&mut client_stream, 0x08).await?; // Address type not supported
                return Err(anyhow::anyhow!("Unsupported address type: {}", atyp));
            }
        };

        if cmd == 0x01 {
            // CONNECT
            info!("CONNECT target: {}", target_addr_str);
            let target_stream = match TcpStream::connect(&target_addr_str).await {
                Ok(stream) => stream,
                Err(e) => {
                    error!("Failed to connect to target {}: {}", target_addr_str, e);
                    reply_error(&mut client_stream, 0x04).await?; // Host unreachable
                    return Err(e.into());
                }
            };

            // Send Success Reply
            client_stream
                .write_all(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0])
                .await?;

            Ok((client_stream, Some(target_stream), None))
        } else if cmd == 0x03 {
            // UDP ASSOCIATE
            info!("UDP ASSOCIATE request from {}", client_stream.peer_addr()?);
            
            // Bind a UDP socket on a random port
            let udp_socket = TokioUdpSocket::bind("0.0.0.0:0").await?;
            let udp_port = udp_socket.local_addr()?.port();
            
            // Reply with the bound address and port
            // BND.ADDR = 0.0.0.0 (or specific IP if needed), BND.PORT = udp_port
            let mut reply = vec![0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0];
            reply.extend_from_slice(&udp_port.to_be_bytes());
            client_stream.write_all(&reply).await?;

            Ok((client_stream, None, Some(udp_socket)))
        } else {
            reply_error(&mut client_stream, 0x07).await?;
            Err(anyhow::anyhow!("Unsupported command: {}", cmd))
        }
    })
    .await
    .context("Handshake/Connection timeout")??;

    if let Some(mut target_stream) = target_stream_opt {
        // TCP Relay
        let (mut client_reader, mut client_writer) = client_stream.split();
        let (mut target_reader, mut target_writer) = target_stream.split();

        let client_to_target = tokio::io::copy(&mut client_reader, &mut target_writer);
        let target_to_client = tokio::io::copy(&mut target_reader, &mut client_writer);

        tokio::select! {
            res = client_to_target => { res.context("Client to target failed")?; }
            res = target_to_client => { res.context("Target to client failed")?; }
        }
    } else if let Some(udp_socket) = udp_associate {
        // UDP Relay
        // Keep TCP connection alive to maintain UDP association
        // And handle UDP packets
        let mut buf = [0u8; 1];
        tokio::select! {
            _ = client_stream.read(&mut buf) => {
                // TCP connection closed or error, close UDP socket
                info!("TCP control connection closed, stopping UDP associate");
            }
            res = handle_udp(udp_socket, client_addr) => {
                res.context("UDP handling failed")?;
            }
        }
    }

    Ok(())
}

async fn handle_udp(socket: TokioUdpSocket, client_addr: SocketAddr) -> Result<()> {
    let mut buf = vec![0u8; 65535];
    let header_offset = 300; // Reserve space for header prepending
    let mut client_udp_addr: Option<SocketAddr> = None;
    let client_ip = client_addr.ip();

    loop {
        // Read into buffer with offset
        let (len, src_addr) = socket.recv_from(&mut buf[header_offset..]).await?;
        let packet = &buf[header_offset..header_offset + len];

        if src_addr.ip() == client_ip {
            // Packet from Client -> Target
            client_udp_addr = Some(src_addr);

            // Parse SOCKS5 UDP Header
            if len < 3 || packet[0] != 0x00 || packet[1] != 0x00 || packet[2] != 0x00 {
                continue; // Invalid header or fragmentation
            }

            let atyp = packet[3];
            let (target_addr, header_len) = match atyp {
                0x01 => { // IPv4
                    if len < 10 { continue; }
                    let ip = std::net::Ipv4Addr::new(packet[4], packet[5], packet[6], packet[7]);
                    let port = u16::from_be_bytes([packet[8], packet[9]]);
                    (SocketAddr::V4(std::net::SocketAddrV4::new(ip, port)), 10)
                }
                0x03 => { // Domain
                    let domain_len = packet[4] as usize;
                    if len < 5 + domain_len + 2 { continue; }
                    let domain = String::from_utf8_lossy(&packet[5..5 + domain_len]);
                    let port = u16::from_be_bytes([packet[5 + domain_len], packet[5 + domain_len + 1]]);
                    match tokio::net::lookup_host(format!("{}:{}", domain, port)).await {
                        Ok(mut addrs) => {
                            if let Some(addr) = addrs.next() {
                                (addr, 5 + domain_len + 2)
                            } else { continue; }
                        }
                        Err(_) => continue,
                    }
                }
                0x04 => { // IPv6
                    if len < 22 { continue; }
                    let ip = std::net::Ipv6Addr::from([
                        packet[4], packet[5], packet[6], packet[7], packet[8], packet[9], packet[10], packet[11],
                        packet[12], packet[13], packet[14], packet[15], packet[16], packet[17], packet[18], packet[19]
                    ]);
                    let port = u16::from_be_bytes([packet[20], packet[21]]);
                    (SocketAddr::V6(std::net::SocketAddrV6::new(ip, port, 0, 0)), 22)
                }
                _ => continue,
            };

            let payload = &packet[header_len..];
            if let Err(e) = socket.send_to(payload, target_addr).await {
                warn!("Failed to forward UDP packet to {}: {}", target_addr, e);
            }

        } else {
            // Packet from Target -> Client
            if let Some(client_udp) = client_udp_addr {
                // Prepend SOCKS5 UDP Header
                let (addr_bytes, port, atyp) = match src_addr {
                    SocketAddr::V4(a) => (a.ip().octets().to_vec(), a.port(), 0x01),
                    SocketAddr::V6(a) => (a.ip().octets().to_vec(), a.port(), 0x04),
                };

                let header_len = 4 + addr_bytes.len() + 2;
                let start_idx = header_offset - header_len;

                buf[start_idx] = 0x00; // RSV
                buf[start_idx + 1] = 0x00; // RSV
                buf[start_idx + 2] = 0x00; // FRAG
                buf[start_idx + 3] = atyp; // ATYP

                for (i, b) in addr_bytes.iter().enumerate() {
                    buf[start_idx + 4 + i] = *b;
                }

                let port_bytes = port.to_be_bytes();
                buf[start_idx + 4 + addr_bytes.len()] = port_bytes[0];
                buf[start_idx + 4 + addr_bytes.len() + 1] = port_bytes[1];

                let total_len = header_len + len;
                if let Err(e) = socket.send_to(&buf[start_idx..start_idx + total_len], client_udp).await {
                    warn!("Failed to send UDP response to client {}: {}", client_udp, e);
                }
            }
        }
    }
}

async fn reply_error(stream: &mut TcpStream, rep: u8) -> Result<()> {
    stream
        .write_all(&[0x05, rep, 0x00, 0x01, 0, 0, 0, 0, 0, 0])
        .await?;
    Ok(())
}
