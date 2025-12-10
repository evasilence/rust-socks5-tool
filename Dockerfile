# Build stage
FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install necessary runtime dependencies (if any, e.g. ca-certificates for SSL)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/rust-socks5-tool /usr/local/bin/rust-socks5-tool

# Expose the default port
EXPOSE 1080

# Run the application
CMD ["rust-socks5-tool"]
