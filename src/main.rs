use anyhow::{Context, Result};
use clap::Parser;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 1080)]
    port: u16,

    /// Address to listen on
    #[arg(short, long, default_value = "0.0.0.0")]
    address: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let bind_addr = format!("{}:{}", args.address, args.port);

    let listener = TcpListener::bind(&bind_addr)
        .await
        .context(format!("Failed to bind to {}", bind_addr))?;

    println!("SOCKS5 server listening on {}", bind_addr);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Accepted connection from {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Error handling client {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(mut client_stream: TcpStream) -> Result<()> {
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

    // We only support NO AUTH (0x00) for now
    if !methods.contains(&0x00) {
        // Send NO ACCEPTABLE METHODS
        client_stream.write_all(&[0x05, 0xFF]).await?;
        return Err(anyhow::anyhow!("No supported authentication methods"));
    }

    // Send Method Selection (NO AUTH)
    client_stream.write_all(&[0x05, 0x00]).await?;

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

    if cmd != 0x01 {
        // We only support CONNECT (0x01)
        // Send Command not supported
        reply_error(&mut client_stream, 0x07).await?;
        return Err(anyhow::anyhow!("Unsupported command: {}", cmd));
    }

    let target_addr = match atyp {
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

    println!("Connecting to target: {}", target_addr);

    let mut target_stream = match TcpStream::connect(&target_addr).await {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Failed to connect to target {}: {}", target_addr, e);
            reply_error(&mut client_stream, 0x04).await?; // Host unreachable (or similar)
            return Err(e.into());
        }
    };

    // Send Success Reply
    // VER | REP | RSV | ATYP | BND.ADDR | BND.PORT
    // 0x05| 0x00| 0x00| 0x01 | 0.0.0.0  | 0
    client_stream
        .write_all(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0])
        .await?;

    // 3. Relay
    let (mut client_reader, mut client_writer) = client_stream.split();
    let (mut target_reader, mut target_writer) = target_stream.split();

    let client_to_target = tokio::io::copy(&mut client_reader, &mut target_writer);
    let target_to_client = tokio::io::copy(&mut target_reader, &mut client_writer);

    tokio::select! {
        res = client_to_target => {
            res.context("Client to target failed")?;
        }
        res = target_to_client => {
            res.context("Target to client failed")?;
        }
    }

    Ok(())
}

async fn reply_error(stream: &mut TcpStream, rep: u8) -> Result<()> {
    stream
        .write_all(&[0x05, rep, 0x00, 0x01, 0, 0, 0, 0, 0, 0])
        .await?;
    Ok(())
}
