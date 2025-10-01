//! # VedDB Client
//!
//! Official Rust client for VedDB - High-performance shared memory KV store with Pub/Sub capabilities.
//!
//! ## Features
//!
//! - **Synchronous and Asynchronous APIs** - Choose between blocking and non-blocking operations
//! - **Connection Pooling** - Efficiently manage multiple connections to VedDB servers
//! - **Automatic Reconnection** - Handle network issues gracefully
//! - **Pipelining** - Send multiple commands without waiting for responses
//! - **Pub/Sub Support** - Subscribe to channels and patterns
//!
//! ## Example
//! ```no_run
//! use veddb_client::{Client, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Connect to a VedDB server
//!     let client = Client::connect("127.0.0.1:50051".parse()?).await?;
//!     
//!     // Set a value
//!     client.set("my_key", "my_value").await?;
//!     
//!     // Get a value
//!     let value: Vec<u8> = client.get("my_key").await?;
//!     println!("Got value: {:?}", value);
//!     
//!     Ok(())
//! }
//! ```

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
