# VedDB Client Libraries

This directory contains client implementations for VedDB in various programming languages.

## Available Clients

### Rust Client (`rust-client/`)
The official Rust client for VedDB providing:
- Async/await support with Tokio
- Connection pooling
- Automatic reconnection
- Type-safe API
- CLI tool for database operations

## Client Features

All VedDB clients provide:
- **Connection Management**: Pooling and automatic reconnection
- **Protocol Support**: Binary protocol for efficient communication
- **Type Safety**: Strongly typed operations (where applicable)
- **Error Handling**: Comprehensive error types
- **Performance**: Optimized for low latency

## Installation

### Rust Client
```bash
cd rust-client
cargo build --release
```

Or add to your `Cargo.toml`:
```toml
[dependencies]
veddb-client = { path = "../path/to/clients/rust-client" }
```

## Usage Example

### Rust
```rust
use veddb_client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to VedDB server
    let addr = "127.0.0.1:50051".parse()?;
    let client = Client::connect(addr).await?;
    
    // Set a value
    client.set("key", "value".as_bytes()).await?;
    
    // Get a value
    let value = client.get("key").await?;
    println!("Value: {}", String::from_utf8_lossy(&value));
    
    Ok(())
}
```

## CLI Tools

### Rust CLI
```bash
# Build the CLI
cd rust-client
cargo build --release --bin veddb-cli

# Use the CLI
./target/release/veddb-cli --server 127.0.0.1:50051 kv get mykey
./target/release/veddb-cli --server 127.0.0.1:50051 kv set mykey "myvalue"
./target/release/veddb-cli --server 127.0.0.1:50051 kv del mykey
./target/release/veddb-cli --server 127.0.0.1:50051 kv list
```

## Protocol

VedDB clients communicate using a binary protocol:
- Fixed-size command/response headers
- Variable-size payloads
- Efficient serialization
- Support for streaming operations

## Contributing

To add a new client implementation:
1. Create a new directory under `clients/` (e.g., `python-client/`)
2. Implement the VedDB protocol
3. Add comprehensive tests
4. Update this README with installation and usage instructions
5. Submit a pull request

## Future Clients

Planned client implementations:
- [ ] Python
- [ ] Go
- [ ] Node.js/TypeScript
- [ ] Java
- [ ] C/C++

## License

MIT License - see LICENSE file in the root directory
