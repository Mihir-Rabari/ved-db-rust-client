//! Protocol types for VedDB client-server communication.

use bytes::{Buf, BufMut, Bytes, BytesMut};
use thiserror::Error;

/// Error type for protocol operations
#[derive(Error, Debug)]
pub enum ProtocolError {
    /// Invalid message format
    #[error("Invalid message format: {0}")]
    InvalidFormat(String),

    /// Invalid opcode
    #[error("Invalid opcode: {0}")]
    InvalidOpCode(u8),

    /// Invalid status code
    #[error("Invalid status code: {0}")]
    InvalidStatusCode(u8),

    /// Message too large
    #[error("Message too large: {0} bytes")]
    MessageTooLarge(usize),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Command opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    /// Ping the server
    Ping = 0x01,
    /// Set a key-value pair
    Set = 0x02,
    /// Get a value by key
    Get = 0x03,
    /// Delete a key
    Delete = 0x04,
    /// Compare and swap
    Cas = 0x05,
    /// Subscribe to a topic
    Subscribe = 0x06,
    /// Unsubscribe from a topic
    Unsubscribe = 0x07,
    /// Publish to a topic
    Publish = 0x08,
    /// Fetch server info
    Info = 0x09,
}

impl TryFrom<u8> for OpCode {
    type Error = ProtocolError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(OpCode::Ping),
            0x02 => Ok(OpCode::Set),
            0x03 => Ok(OpCode::Get),
            0x04 => Ok(OpCode::Delete),
            0x05 => Ok(OpCode::Cas),
            0x06 => Ok(OpCode::Subscribe),
            0x07 => Ok(OpCode::Unsubscribe),
            0x08 => Ok(OpCode::Publish),
            0x09 => Ok(OpCode::Info),
            _ => Err(ProtocolError::InvalidOpCode(value)),
        }
    }
}

/// Response status codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    /// Operation succeeded
    Ok = 0x00,
    /// Key not found
    NotFound = 0x01,
    /// Version mismatch (for CAS operations)
    VersionMismatch = 0x02,
    /// Invalid arguments
    InvalidArgs = 0x03,
    /// Internal server error
    InternalError = 0x04,
    /// Not authenticated
    Unauthorized = 0x05,
    /// Operation not supported
    NotSupported = 0x06,
}

impl TryFrom<u8> for StatusCode {
    type Error = ProtocolError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(StatusCode::Ok),
            0x01 => Ok(StatusCode::NotFound),
            0x02 => Ok(StatusCode::VersionMismatch),
            0x03 => Ok(StatusCode::InvalidArgs),
            0x04 => Ok(StatusCode::InternalError),
            0x05 => Ok(StatusCode::Unauthorized),
            0x06 => Ok(StatusCode::NotSupported),
            _ => Err(ProtocolError::InvalidStatusCode(value)),
        }
    }
}

/// Command header (24 bytes)
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CommandHeader {
    /// Operation code
    pub opcode: u8,
    /// Command flags
    pub flags: u8,
    /// Reserved for future use
    pub reserved: u16,
    /// Client-local sequence ID
    pub seq: u32,
    /// Key length in bytes
    pub key_len: u32,
    /// Value length in bytes
    pub value_len: u32,
    /// Extra data (version for CAS, TTL, etc.)
    pub extra: u64,
}

impl CommandHeader {
    /// Create a new command header
    pub fn new(opcode: OpCode, seq: u32) -> Self {
        Self {
            opcode: opcode as u8,
            flags: 0,
            reserved: 0,
            seq,
            key_len: 0,
            value_len: 0,
            extra: 0,
        }
    }

    /// Set the key and value lengths
    pub fn with_lengths(mut self, key_len: u32, value_len: u32) -> Self {
        self.key_len = key_len;
        self.value_len = value_len;
        self
    }

    /// Set extra data
    pub fn with_extra(mut self, extra: u64) -> Self {
        self.extra = extra;
        self
    }

    /// Set a flag
    pub fn with_flag(mut self, flag: u8) -> Self {
        self.flags |= flag;
        self
    }
}

/// Command structure
#[derive(Debug, Clone)]
pub struct Command {
    /// Command header
    pub header: CommandHeader,
    /// Key (if any)
    pub key: Bytes,
    /// Value (if any)
    pub value: Bytes,
}

impl Command {
    /// Create a new command
    pub fn new(header: CommandHeader, key: impl Into<Bytes>, value: impl Into<Bytes>) -> Self {
        let key = key.into();
        let value = value.into();
        Self {
            header: header.with_lengths(key.len() as u32, value.len() as u32),
            key,
            value,
        }
    }

    /// Create a PING command
    pub fn ping(seq: u32) -> Self {
        Self::new(
            CommandHeader::new(OpCode::Ping, seq),
            Bytes::new(),
            Bytes::new(),
        )
    }

    /// Create a SET command
    pub fn set<K, V>(seq: u32, key: K, value: V) -> Self
    where
        K: Into<Bytes>,
        V: Into<Bytes>,
    {
        Self::new(CommandHeader::new(OpCode::Set, seq), key, value)
    }

    /// Create a GET command
    pub fn get<K>(seq: u32, key: K) -> Self
    where
        K: Into<Bytes>,
    {
        Self::new(CommandHeader::new(OpCode::Get, seq), key, Bytes::new())
    }

    /// Create a DELETE command
    pub fn delete<K>(seq: u32, key: K) -> Self
    where
        K: Into<Bytes>,
    {
        Self::new(CommandHeader::new(OpCode::Delete, seq), key, Bytes::new())
    }

    /// Create a CAS (Compare-And-Swap) command
    pub fn cas<K, V>(seq: u32, key: K, expected_version: u64, value: V) -> Self
    where
        K: Into<Bytes>,
        V: Into<Bytes>,
    {
        Self::new(
            CommandHeader::new(OpCode::Cas, seq).with_extra(expected_version),
            key,
            value,
        )
    }

    /// Serialize the command to bytes
    pub fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::with_capacity(24 + self.key.len() + self.value.len());

        // Write header (24 bytes)
        buf.put_u8(self.header.opcode);
        buf.put_u8(self.header.flags);
        buf.put_u16(self.header.reserved);
        buf.put_u32(self.header.seq);
        buf.put_u32(self.header.key_len);
        buf.put_u32(self.header.value_len);
        buf.put_u64(self.header.extra);

        // Write key and value
        buf.extend_from_slice(&self.key);
        buf.extend_from_slice(&self.value);

        buf.freeze()
    }
}

/// Response header (16 bytes)
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ResponseHeader {
    /// Status code
    pub status: u8,
    /// Response flags
    pub flags: u8,
    /// Reserved
    pub reserved: u16,
    /// Sequence number
    pub seq: u32,
    /// Payload length
    pub payload_len: u32,
    /// Reserved for future use
    pub reserved2: u32,
}

impl ResponseHeader {
    /// Create a new response header
    pub fn new(status: StatusCode, seq: u32) -> Self {
        Self {
            status: status as u8,
            flags: 0,
            reserved: 0,
            seq,
            payload_len: 0,
            reserved2: 0,
        }
    }

    /// Set the payload length
    pub fn with_payload_len(mut self, len: u32) -> Self {
        self.payload_len = len;
        self
    }
}

/// Response structure
#[derive(Debug, Clone)]
pub struct Response {
    /// Response header
    pub header: ResponseHeader,
    /// Response payload
    pub payload: Bytes,
}

impl Response {
    /// Create a new response
    pub fn new(header: ResponseHeader, payload: impl Into<Bytes>) -> Self {
        let payload = payload.into();
        Self {
            header: header.with_payload_len(payload.len() as u32),
            payload,
        }
    }

    /// Create a success response
    pub fn ok(seq: u32, payload: impl Into<Bytes>) -> Self {
        Self::new(ResponseHeader::new(StatusCode::Ok, seq), payload)
    }

    /// Create a not found response
    pub fn not_found(seq: u32) -> Self {
        Self::new(ResponseHeader::new(StatusCode::NotFound, seq), Bytes::new())
    }

    /// Create an error response
    pub fn error(seq: u32) -> Self {
        Self::new(
            ResponseHeader::new(StatusCode::InternalError, seq),
            Bytes::new(),
        )
    }

    /// Deserialize a response from bytes
    pub fn from_bytes(mut bytes: &[u8]) -> Result<Self, ProtocolError> {
        if bytes.len() < 16 {
            return Err(ProtocolError::InvalidFormat("response too short".into()));
        }

        // Read header (16 bytes)
        let status = StatusCode::try_from(bytes.get_u8())?;
        let flags = bytes.get_u8();
        let reserved = bytes.get_u16();
        let seq = bytes.get_u32();
        let payload_len = bytes.get_u32() as usize;
        let reserved2 = bytes.get_u32();

        // Check payload length
        if bytes.remaining() < payload_len {
            return Err(ProtocolError::InvalidFormat(
                "invalid payload length".into(),
            ));
        }

        // Read payload
        let payload = bytes.copy_to_bytes(payload_len);

        Ok(Self {
            header: ResponseHeader {
                status: status as u8,
                flags,
                reserved,
                seq,
                payload_len: payload_len as u32,
                reserved2,
            },
            payload,
        })
    }

    /// Check if the response indicates success
    pub fn is_ok(&self) -> bool {
        matches!(StatusCode::try_from(self.header.status), Ok(StatusCode::Ok))
    }

    /// Get the status code
    pub fn status(&self) -> StatusCode {
        StatusCode::try_from(self.header.status).unwrap_or(StatusCode::InternalError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn test_command_serialization() {
        let cmd = Command::set(1, "key", "value");
        let bytes = cmd.to_bytes();

        assert_eq!(bytes[0], OpCode::Set as u8);
        assert_eq!(&bytes[24..27], b"key");
        assert_eq!(&bytes[27..32], b"value");
    }

    #[test]
    fn test_response_deserialization() {
        let mut buf = BytesMut::new();
        buf.put_u8(StatusCode::Ok as u8); // status
        buf.put_u8(0); // flags
        buf.put_u16(0); // reserved
        buf.put_u32(42); // seq
        buf.put_u32(5); // payload_len
        buf.put_u32(0); // reserved2
        buf.extend_from_slice(b"hello"); // payload

        let resp = Response::from_bytes(&buf).unwrap();
        assert!(resp.is_ok());
        assert_eq!(resp.header.seq, 42);
        assert_eq!(&resp.payload[..], b"hello");
    }
}
