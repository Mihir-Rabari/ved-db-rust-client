# VedDB Rust Client v0.0.11

**Official Rust client and CLI for VedDB Server**

A fast, easy-to-use Rust client library and command-line interface for interacting with VedDB Server. Built with async/await and designed for high performance.

![Windows](https://img.shields.io/badge/platform-windows-blue)
![Rust](https://img.shields.io/badge/rust-1.75+-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## ✨ Features

- **🚀 Async/Await**: Built on Tokio for high-performance async I/O
- **🔌 Connection Pooling**: Efficient connection management
- **📝 CLI Tool**: Easy-to-use command-line interface
- **🎯 Type-Safe**: Full Rust type safety and error handling
- **📊 Multiple Output Formats**: Table, JSON, and raw output
- **⚡ Fast**: Sub-millisecond operation latency

## 🚀 Quick Start

### Download & Installation (Windows)

VedDB CLI is currently tested and supported on **Windows**. You can download the pre-built executable:

**Option 1: Download from Website**
- Visit our website and download the latest Windows CLI `.exe`

**Option 2: GitHub Releases**
- Go to [Releases](https://github.com/Mihir-Rabari/ved-db-rust-client/releases)
- Download `veddb-cli-v0.0.11-windows.exe`

### Basic Usage

```
# Ping the server
veddb-cli.exe ping

# Set a key-value pair
veddb-cli.exe kv set name "John Doe"

# Get a value
veddb-cli.exe kv get name

# List all keys
veddb-cli.exe kv list

# Delete a key
veddb-cli.exe kv del name
```

## 📖 CLI Commands

### Global Options

```
veddb-cli.exe [OPTIONS] <COMMAND>

Options:
  -s, --server <SERVER>  Server address [default: 127.0.0.1:50051]
  -f, --format <FORMAT>  Output format [default: table] [values: table, json, raw]
  -v, --verbose          Enable verbose output
  -h, --help             Print help
  -V, --version          Print version
```

### KV Commands

#### Set a Key
```
veddb-cli.exe kv set <KEY> <VALUE>

Examples:
veddb-cli.exe kv set name "Alice"
veddb-cli.exe kv set age 25
veddb-cli.exe kv set city "New York"
```

#### Get a Key
```
veddb-cli.exe kv get <KEY>

Example output:
+------+-------+
| Key  | Value |
+------+-------+
| name | Alice |
+------+-------+
```

#### Delete a Key
```
veddb-cli.exe kv del <KEY>
```

#### List All Keys
```
veddb-cli.exe kv list

Example output:
+------+
| Keys |
+------+
| name |
| age  |
| city |
+------+
```

### Ping Command

Check server connectivity:
```
veddb-cli.exe ping

Example output:
+--------+---------+
| Status | Latency |
+--------+---------+
| pong   | 0 ms    |
+--------+---------+
```

### Output Formats

**Table Format (Default)**
```
veddb-cli.exe kv get name
```

**JSON Format**
```
veddb-cli.exe -f json kv get name
```

**Raw Format**
```
veddb-cli.exe -f raw kv get name
```

## 🔧 Using as a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
veddb-client = "0.0.11"
tokio = { version = "1", features = ["full"] }
```

### Basic Example

```rust
use veddb_client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to server
    let client = Client::connect("127.0.0.1:50051").await?;
    
    // Ping server
    client.ping().await?;
    println!("Server is alive!");
    
    // Set a key
    client.set("name", "Alice").await?;
    
    // Get a key
    let value = client.get("name").await?;
    println!("Value: {}", String::from_utf8_lossy(&value));
    
    // List all keys
    let keys = client.list_keys().await?;
    println!("Keys: {:?}", keys);
    
    // Delete a key
    client.delete("name").await?;
    
    Ok(())
}
```

### Connection Pooling

```rust
use veddb_client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with connection pool
    let client = Client::with_pool_size("127.0.0.1:50051", 10).await?;
    
    // Use client (connections are automatically managed)
    client.set("key", "value").await?;
    
    Ok(())
}
```

## 🛠️ Development

### Building from Source

**Prerequisites:**
- Rust 1.75 or later ([Install Rust](https://rustup.rs/))
- Windows 10/11

```
git clone https://github.com/Mihir-Rabari/ved-db-rust-client.git
cd ved-db-rust-client
cargo build --release
```

CLI binary will be at: `target\release\veddb-cli.exe`

### Running Tests

```
cargo test
```

### Building Just the CLI

```
cargo build --release --bin veddb-cli
```

## 📊 Performance

- **Latency**: < 1ms for most operations
- **Connection Pooling**: Reuses connections for better performance
- **Async I/O**: Non-blocking operations with Tokio

## 🔌 Protocol Details

The client implements the VedDB binary protocol:

- **Little-endian** encoding for all integers
- **Command format**: 24-byte header + payload
- **Response format**: 20-byte header + payload
- **Automatic retries**: On connection failures
- **Timeout handling**: Configurable request timeouts

## 🗺️ Roadmap

### Current (v0.0.11)
- ✅ Basic KV operations (SET, GET, DELETE, LIST)
- ✅ PING command
- ✅ CLI with table/JSON/raw output
- ✅ Connection pooling
- ✅ Async/await support

### Planned (v0.1.x)
- ⏳ Pub/Sub support
- ⏳ TTL operations
- ⏳ Batch operations
- ⏳ Transaction support
- ⏳ Pattern matching for LIST

### Future (v1.0.x)
- ⏳ TLS/SSL support
- ⏳ Authentication
- ⏳ Compression
- ⏳ Streaming responses

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🤝 Contributing

Contributions welcome! Please open an issue or PR on GitHub.

## 📧 Contact

- **Email**: mihirrabari2604@gmail.com
- **Instagram**: @mihirrabariii

---

**Built with ❤️ in Rust**
