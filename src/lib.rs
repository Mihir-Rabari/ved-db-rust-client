//! # VedDB Rust Client
//!
//! [![Crates.io](https://img.shields.io/crates/v/veddb-client.svg)](https://crates.io/crates/veddb-client)
//! [![Documentation](https://docs.rs/veddb-client/badge.svg)](https://docs.rs/veddb-client)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
//!
//! **Official Rust client library and CLI for VedDB** - A fast, lightweight in-memory key-value database.
//!
//! This crate provides both a **Rust library** for embedding in your applications and a **CLI tool** 
//! (`veddb-cli`) for interactive database operations.
//!
//! ## 🚀 Quick Start
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! veddb-client = "0.0.11"
//! tokio = { version = "1", features = ["full"] }
//! ```
//!
//! ### Basic Example
//!
//! ```no_run
//! use veddb_client::{Client, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Connect to VedDB server
//!     let client = Client::connect("127.0.0.1:50051").await?;
//!     
//!     // Ping the server
//!     client.ping().await?;
//!     println!("Server is alive!");
//!     
//!     // Set a key-value pair
//!     client.set("name", "Alice").await?;
//!     
//!     // Get a value
//!     let value = client.get("name").await?;
//!     println!("Value: {}", String::from_utf8_lossy(&value));
//!     
//!     // List all keys
//!     let keys = client.list_keys().await?;
//!     println!("Keys: {:?}", keys);
//!     
//!     // Delete a key
//!     client.delete("name").await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## ✨ Features
//!
//! - **Async/Await** - Built on Tokio for high-performance async I/O
//! - **Connection Pooling** - Efficient connection management and reuse
//! - **Type-Safe** - Full Rust type safety and error handling
//! - **All Operations** - PING, SET, GET, DELETE, LIST commands
//! - **CLI Tool** - Command-line interface included (`veddb-cli`)
//!
//! ## 📖 Usage Examples
//!
//! ### Connection Pooling
//!
//! ```no_run
//! use veddb_client::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client with connection pool (10 connections)
//!     let client = Client::with_pool_size("127.0.0.1:50051", 10).await?;
//!     
//!     // Connections are automatically managed
//!     client.set("key1", "value1").await?;
//!     client.set("key2", "value2").await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Working with Binary Data
//!
//! ```no_run
//! use veddb_client::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::connect("127.0.0.1:50051").await?;
//!     
//!     // Store binary data
//!     let data = vec![0x01, 0x02, 0x03, 0x04];
//!     client.set("binary_key", &data).await?;
//!     
//!     // Retrieve binary data
//!     let retrieved = client.get("binary_key").await?;
//!     assert_eq!(retrieved, data);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Error Handling
//!
//! ```no_run
//! use veddb_client::{Client, Error};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::connect("127.0.0.1:50051").await.unwrap();
//!     
//!     match client.get("nonexistent_key").await {
//!         Ok(value) => println!("Found: {:?}", value),
//!         Err(Error::Server(msg)) if msg.contains("NotFound") => {
//!             println!("Key not found");
//!         }
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//! }
//! ```
//!
//! ## 🖥️ CLI Tool
//!
//! This crate includes `veddb-cli`, a command-line interface for VedDB:
//!
//! ```bash
//! # Ping server
//! veddb-cli ping
//!
//! # Set a key
//! veddb-cli kv set name Alice
//!
//! # Get a key
//! veddb-cli kv get name
//!
//! # List all keys
//! veddb-cli kv list
//!
//! # Delete a key
//! veddb-cli kv del name
//! ```
//!
//! ## 🔌 Protocol
//!
//! VedDB uses a simple binary protocol over TCP:
//!
//! - **Little-endian** encoding for all integers
//! - **Command format**: 24-byte header + payload
//! - **Response format**: 20-byte header + payload
//!
//! ### Supported Operations
//!
//! | OpCode | Command | Description |
//! |--------|---------|-------------|
//! | `0x01` | PING | Health check |
//! | `0x02` | SET | Store key-value pair |
//! | `0x03` | GET | Retrieve value by key |
//! | `0x04` | DELETE | Remove key |
//! | `0x09` | LIST | List all keys |
//!
//! ## 🔗 Related
//!
//! - **Server**: [ved-db-server](https://github.com/Mihir-Rabari/ved-db-server) - VedDB Server v0.1.21
//! - **Repository**: [GitHub](https://github.com/Mihir-Rabari/ved-db-rust-client)
//!
//! ## 📄 License
//!
//! This project is licensed under the MIT License - see the [LICENSE](https://github.com/Mihir-Rabari/ved-db-rust-client/blob/main/LICENSE) file for details.

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![forbid(unsafe_code)]

mod connection;
mod error;
mod types;

pub use connection::{Client, ClientBuilder, Connection, ConnectionPool};
pub use error::Error;
pub use types::{Command, Response, StatusCode};

/// Custom result type for VedDB operations
pub type Result<T> = std::result::Result<T, Error>;

/// Re-export of the `bytes` crate for convenience
pub use bytes;

/// Re-export of the `tracing` crate for convenience
#[cfg(feature = "tracing")]
pub use tracing;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_error_conversion() {
        // Test that we can convert from io::Error
        let io_error = std::io::Error::new(std::io::ErrorKind::Other, "test");
        let error: Error = io_error.into();
        assert!(matches!(error, Error::Io(_)));
        
        // Test that we can convert from string
        let error: Error = "test error".into();
        assert!(matches!(error, Error::Other(_)));
    }
}
