//! Connection handling for VedDB client

use std::net::SocketAddr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

use bytes::Bytes;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::timeout;
use tracing::{debug, error, info, trace};

use crate::types::{Command, Response};
use crate::{Error, Result};

/// Default connection timeout
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
/// Default request timeout
const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
/// Maximum frame size (16MB)
const MAX_FRAME_SIZE: usize = 16 * 1024 * 1024;

/// A connection to a VedDB server
#[derive(Debug)]
pub struct Connection {
    /// The underlying TCP stream
    stream: Mutex<TcpStream>,
    /// Server address
    addr: SocketAddr,
    /// Next sequence number
    next_seq: AtomicU32,
    /// Connection timeout
    connect_timeout: Duration,
    /// Request timeout
    request_timeout: Duration,
}

impl Connection {
    /// Create a new connection to the specified address
    pub async fn connect(addr: impl Into<SocketAddr>) -> Result<Self> {
        Self::connect_with_timeout(addr, DEFAULT_CONNECT_TIMEOUT, DEFAULT_REQUEST_TIMEOUT).await
    }

    /// Create a new connection with custom timeouts
    pub async fn connect_with_timeout(
        addr: impl Into<SocketAddr>,
        connect_timeout: Duration,
        request_timeout: Duration,
    ) -> Result<Self> {
        let addr = addr.into();
        info!("Connecting to VedDB server at {}", addr);

        let stream = timeout(connect_timeout, TcpStream::connect(&addr))
            .await
            .map_err(Error::Timeout)??;

        info!("Connected to VedDB server at {}", addr);

        Ok(Self {
            stream: Mutex::new(stream),
            addr,
            next_seq: AtomicU32::new(1),
            connect_timeout,
            request_timeout,
        })
    }

    /// Get the next sequence number
    fn next_seq(&self) -> u32 {
        self.next_seq.fetch_add(1, Ordering::SeqCst)
    }

    /// Execute a command and return the response
    pub async fn execute(&self, cmd: Command) -> Result<Response> {
        let seq = cmd.header.seq;
        debug!("Executing command: {:?} (seq={})", cmd.header.opcode, seq);

        let mut stream = self.stream.lock().await;

        // Send the command
        let cmd_bytes = cmd.to_bytes();
        trace!("Sending {} bytes: {:?}", cmd_bytes.len(), cmd_bytes);

        timeout(self.request_timeout, stream.write_all(&cmd_bytes))
            .await
            .map_err(Error::Timeout)??;

        // Read the response header (16 bytes)
        let mut header_buf = [0u8; 16];
        timeout(self.request_timeout, stream.read_exact(&mut header_buf))
            .await
            .map_err(Error::Timeout)??;

        // Parse the header
        let payload_len =
            u32::from_le_bytes([header_buf[8], header_buf[9], header_buf[10], header_buf[11]]);

        if payload_len as usize > MAX_FRAME_SIZE {
            return Err(Error::Protocol(format!(
                "Response too large: {} bytes (max: {})",
                payload_len, MAX_FRAME_SIZE
            )));
        }

        // Read the payload
        let mut payload = vec![0u8; payload_len as usize];
        if payload_len > 0 {
            timeout(self.request_timeout, stream.read_exact(&mut payload))
                .await
                .map_err(Error::Timeout)??;
        }

        // Combine header and payload for parsing
        let mut response_bytes = Vec::with_capacity(16 + payload_len as usize);
        response_bytes.extend_from_slice(&header_buf);
        response_bytes.extend_from_slice(&payload);

        let response = Response::from_bytes(&response_bytes)
            .map_err(|e| Error::Protocol(format!("Invalid response: {}", e)))?;

        // Verify sequence number
        if response.header.seq != seq {
            return Err(Error::Protocol(format!(
                "Sequence number mismatch: expected {}, got {}",
                seq, response.header.seq
            )));
        }

        // Check for server errors
        if !response.is_ok() {
            let status = response.status();
            let error_msg = String::from_utf8_lossy(&response.payload).into_owned();
            return Err(Error::Server(format!(
                "Server error: {:?}: {}",
                status, error_msg
            )));
        }

        Ok(response)
    }

    /// Ping the server
    pub async fn ping(&self) -> Result<()> {
        let seq = self.next_seq();
        let cmd = Command::ping(seq);
        self.execute(cmd).await?;
        Ok(())
    }

    /// Set a key-value pair
    pub async fn set<K, V>(&self, key: K, value: V) -> Result<()>
    where
        K: Into<Bytes>,
        V: Into<Bytes>,
    {
        let seq = self.next_seq();
        let cmd = Command::set(seq, key, value);
        self.execute(cmd).await?;
        Ok(())
    }

    /// Get a value by key
    pub async fn get<K>(&self, key: K) -> Result<Bytes>
    where
        K: Into<Bytes>,
    {
        let seq = self.next_seq();
        let cmd = Command::get(seq, key);
        let response = self.execute(cmd).await?;
        Ok(response.payload)
    }

    /// Delete a key
    pub async fn delete<K>(&self, key: K) -> Result<()>
    where
        K: Into<Bytes>,
    {
        let seq = self.next_seq();
        let cmd = Command::delete(seq, key);
        self.execute(cmd).await?;
        Ok(())
    }

    /// Compare and swap a value
    pub async fn cas<K, V>(&self, key: K, expected_version: u64, value: V) -> Result<()>
    where
        K: Into<Bytes>,
        V: Into<Bytes>,
    {
        let seq = self.next_seq();
        let cmd = Command::cas(seq, key, expected_version, value);
        self.execute(cmd).await?;
        Ok(())
    }
}

/// A client for interacting with a VedDB server
#[derive(Clone, Debug)]
pub struct Client {
    /// The connection pool
    pool: ConnectionPool,
}

impl Client {
    /// Create a new client connected to the specified address
    pub async fn connect(addr: impl Into<SocketAddr>) -> Result<Self> {
        let pool = ConnectionPool::new(addr, 1).await?;
        Ok(Self { pool })
    }

    /// Create a new client with a connection pool of the specified size
    pub async fn with_pool_size(addr: impl Into<SocketAddr>, pool_size: usize) -> Result<Self> {
        let pool = ConnectionPool::new(addr, pool_size).await?;
        Ok(Self { pool })
    }

    /// Ping the server
    pub async fn ping(&self) -> Result<()> {
        self.pool.get().await?.ping().await
    }

    /// Set a key-value pair
    pub async fn set<K, V>(&self, key: K, value: V) -> Result<()>
    where
        K: Into<Bytes>,
        V: Into<Bytes>,
    {
        self.pool.get().await?.set(key, value).await
    }

    /// Get a value by key
    pub async fn get<K>(&self, key: K) -> Result<Bytes>
    where
        K: Into<Bytes>,
    {
        self.pool.get().await?.get(key).await
    }

    /// Delete a key
    pub async fn delete<K>(&self, key: K) -> Result<()>
    where
        K: Into<Bytes>,
    {
        self.pool.get().await?.delete(key).await
    }

    /// Compare and swap a value
    pub async fn cas<K, V>(&self, key: K, expected_version: u64, value: V) -> Result<()>
    where
        K: Into<Bytes>,
        V: Into<Bytes>,
    {
        self.pool
            .get()
            .await?
            .cas(key, expected_version, value)
            .await
    }
}

/// A connection pool for managing multiple connections to a VedDB server
#[derive(Debug, Clone)]
pub struct ConnectionPool {
    /// The server address
    addr: SocketAddr,
    /// The connection pool receiver
    pool: async_channel::Receiver<Connection>,
    /// The connection pool sender
    pool_sender: async_channel::Sender<Connection>,
    /// The number of connections in the pool
    size: usize,
}

impl ConnectionPool {
    /// Create a new connection pool
    pub async fn new(addr: impl Into<SocketAddr>, size: usize) -> Result<Self> {
        let addr = addr.into();
        let (tx, rx) = async_channel::bounded(size);

        // Initialize connections
        for _ in 0..size {
            let conn = Connection::connect(addr).await?;
            tx.send(conn)
                .await
                .map_err(|e| Error::Connection(e.to_string()))?;
        }

        Ok(Self {
            addr,
            pool: rx,
            pool_sender: tx,
            size,
        })
    }

    /// Get a connection from the pool
    pub async fn get(&self) -> Result<ConnectionGuard> {
        let conn = self
            .pool
            .recv()
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;
        Ok(ConnectionGuard {
            conn: Some(conn),
            pool: self.pool_sender.clone(),
        })
    }

    /// Get the number of connections in the pool
    pub fn size(&self) -> usize {
        self.size
    }
}

/// A guard that returns a connection to the pool when dropped
pub struct ConnectionGuard {
    /// The connection
    conn: Option<Connection>,
    /// The connection pool
    pool: async_channel::Sender<Connection>,
}

impl ConnectionGuard {
    /// Get a reference to the underlying connection
    pub fn connection(&self) -> &Connection {
        self.conn.as_ref().unwrap()
    }

    /// Get a mutable reference to the underlying connection
    pub fn connection_mut(&mut self) -> &mut Connection {
        self.conn.as_mut().unwrap()
    }
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        if let Some(conn) = self.conn.take() {
            let pool = self.pool.clone();
            tokio::spawn(async move {
                if let Err(e) = pool.send(conn).await {
                    error!("Failed to return connection to pool: {}", e);
                }
            });
        }
    }
}

impl std::ops::Deref for ConnectionGuard {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        self.connection()
    }
}

impl std::ops::DerefMut for ConnectionGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.connection_mut()
    }
}

/// A builder for configuring and creating a client
#[derive(Debug, Clone)]
pub struct ClientBuilder {
    /// The server address
    addr: SocketAddr,
    /// The connection pool size
    pool_size: usize,
    /// The connection timeout
    connect_timeout: Duration,
    /// The request timeout
    request_timeout: Duration,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            addr: ([127, 0, 0, 1], 50051).into(),
            pool_size: 10,
            connect_timeout: DEFAULT_CONNECT_TIMEOUT,
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
        }
    }
}

impl ClientBuilder {
    /// Create a new builder with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the server address
    pub fn addr(mut self, addr: impl Into<SocketAddr>) -> Self {
        self.addr = addr.into();
        self
    }

    /// Set the connection pool size
    pub fn pool_size(mut self, size: usize) -> Self {
        self.pool_size = size;
        self
    }

    /// Set the connection timeout
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Set the request timeout
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    /// Build and connect the client
    pub async fn connect(self) -> Result<Client> {
        let pool = ConnectionPool::new(self.addr, self.pool_size).await?;
        Ok(Client { pool })
    }
}
