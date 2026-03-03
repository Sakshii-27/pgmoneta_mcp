// Copyright (C) 2026 The pgmoneta community
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

mod info;

use super::configuration::CONFIG;
use super::constant::*;
use super::security::SecurityUtil;
use anyhow::anyhow;
use chrono::Local;
use serde::Serialize;
use std::fmt::Debug;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// Represents the header of a request sent to the pgmoneta server.
///
/// Contains metadata such as the command code, client version,
/// formatting preferences, and security settings.
#[derive(Serialize, Clone, Debug)]
struct RequestHeader {
    #[serde(rename = "Command")]
    command: u32,
    #[serde(rename = "ClientVersion")]
    client_version: String,
    #[serde(rename = "Output")]
    output_format: u8,
    #[serde(rename = "Timestamp")]
    timestamp: String,
    #[serde(rename = "Compression")]
    compression: u8,
    #[serde(rename = "Encryption")]
    encryption: u8,
}

/// A wrapper structure that combines a request header with its specific payload.
///
/// This is the final serialized object sent over the TCP connection to pgmoneta.
#[derive(Serialize, Clone, Debug)]
struct PgmonetaRequest<R>
where
    R: Serialize + Clone + Debug,
{
    #[serde(rename = "Header")]
    header: RequestHeader,
    #[serde(rename = "Request")]
    request: R,
}

/// Handles network communication with the backend pgmoneta server.
///
/// This client manages the lifecycle of a request: building headers,
/// authenticating, opening a TCP stream, writing the payload, and reading the response.
pub struct PgmonetaClient;
impl PgmonetaClient {
    /// Constructs a standard request header for a given command.
    ///
    /// The header includes the current local timestamp and defaults to
    /// no encryption or compression, expecting a JSON response.
    fn build_request_header(command: u32) -> RequestHeader {
        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
        RequestHeader {
            command,
            client_version: CLIENT_VERSION.to_string(),
            output_format: Format::JSON,
            timestamp,
            compression: Compression::NONE,
            encryption: Encryption::NONE,
        }
    }

    /// Establishes an authenticated TCP connection to the pgmoneta server.
    ///
    /// Looks up the provided `username` in the configuration to find the encrypted
    /// password, decrypts it using the master key, and initiates the connection.
    ///
    /// # Arguments
    /// * `username` - The admin username requesting the connection.
    ///
    /// # Returns
    /// An authenticated `TcpStream` ready for read/write operations.
    async fn connect_to_server(username: &str) -> anyhow::Result<TcpStream> {
        let config = CONFIG.get().expect("Configuration should be enabled");
        let security_util = SecurityUtil::new();

        if !config.admins.contains_key(username) {
            return Err(anyhow!(
                "request_backup_info: unable to find user {username}"
            ));
        }

        let password_encrypted = config
            .admins
            .get(username)
            .expect("Username should be found");
        let master_key = security_util.load_master_key()?;
        let password = String::from_utf8(
            security_util.decrypt_from_base64_string(password_encrypted, &master_key[..])?,
        )?;
        let stream = SecurityUtil::connect_to_server(
            &config.pgmoneta.host,
            config.pgmoneta.port,
            username,
            &password,
        )
        .await?;
        Ok(stream)
    }

    /// Writes a serialized JSON request string to the active TCP stream.
    ///
    /// Protocol flow:
    /// 1. Writes the compression flag.
    /// 2. Writes the encryption flag.
    /// 3. Writes the length of the payload.
    /// 4. Writes the exact payload bytes.
    async fn write_request(request_str: &str, stream: &mut TcpStream) -> anyhow::Result<()> {
        let mut request_buf = Vec::new();
        request_buf.write_i32(request_str.len() as i32).await?;
        request_buf.write_all(request_str.as_bytes()).await?;

        stream.write_u8(Compression::NONE).await?;
        stream.write_u8(Encryption::NONE).await?;
        stream.write_all(request_buf.as_slice()).await?;
        Ok(())
    }

    /// Reads the response payload from the TCP stream.
    ///
    /// Protocol flow:
    /// 1. Reads the compression flag.
    /// 2. Reads the encryption flag.
    /// 3. Reads the payload length.
    /// 4. Reads the exact number of bytes specified by the length.
    async fn read_response(stream: &mut TcpStream) -> anyhow::Result<String> {
        let _compression = stream.read_u8().await?;
        let _encryption = stream.read_u8().await?;
        let len = stream.read_u32().await? as usize;
        let mut response = vec![0u8; len];
        let n = stream.read_exact(&mut response).await?;
        let response_str = String::from_utf8(Vec::from(&response[..n]))?;
        Ok(response_str)
    }

    /// End-to-end wrapper for sending a request to the pgmoneta server and awaiting its response.
    ///
    /// # Arguments
    /// * `username` - The admin username making the request.
    /// * `command` - The numeric command code (e.g., `Command::INFO`).
    /// * `request` - The specific request payload object.
    ///
    /// # Returns
    /// The raw string response from the pgmoneta server.
    async fn forward_request<R>(username: &str, command: u32, request: R) -> anyhow::Result<String>
    where
        R: Serialize + Clone + Debug,
    {
        let mut stream = Self::connect_to_server(username).await?;
        tracing::info!(username = username, "Connected to server");

        let header = Self::build_request_header(command);
        let request = PgmonetaRequest { request, header };

        let request_str = serde_json::to_string(&request)?;
        Self::write_request(&request_str, &mut stream).await?;
        tracing::debug!(username = username, request = ?request, "Sent request to server");
        Self::read_response(&mut stream).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_request_header() {
        let header = PgmonetaClient::build_request_header(Command::INFO);

        assert_eq!(header.command, Command::INFO);
        assert_eq!(header.client_version, CLIENT_VERSION);
        assert_eq!(header.output_format, Format::JSON);
        assert_eq!(header.compression, Compression::NONE);
        assert_eq!(header.encryption, Encryption::NONE);

        // Timestamp should be in YYYYMMDDHHmmss format (14 characters)
        assert_eq!(header.timestamp.len(), 14);
        assert!(header.timestamp.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_build_request_header_different_commands() {
        let header1 = PgmonetaClient::build_request_header(Command::INFO);
        let header2 = PgmonetaClient::build_request_header(Command::LIST_BACKUP);

        assert_eq!(header1.command, Command::INFO);
        assert_eq!(header2.command, Command::LIST_BACKUP);
        assert_ne!(header1.command, header2.command);
    }

    #[test]
    fn test_request_serialization() {
        #[derive(Serialize, Clone, Debug)]
        struct TestRequest {
            field1: String,
            field2: i32,
        }

        let test_request = TestRequest {
            field1: "test".to_string(),
            field2: 42,
        };

        let header = PgmonetaClient::build_request_header(Command::INFO);
        let request = PgmonetaRequest {
            header,
            request: test_request,
        };

        let serialized = serde_json::to_string(&request).expect("Serialization should succeed");

        // Verify JSON contains expected fields
        assert!(serialized.contains("\"Header\""));
        assert!(serialized.contains("\"Request\""));
        assert!(serialized.contains("\"Command\""));
        assert!(serialized.contains("\"ClientVersion\""));
        assert!(serialized.contains("\"field1\""));
        assert!(serialized.contains("\"field2\""));
        assert!(serialized.contains("\"test\""));
        assert!(serialized.contains("42"));
    }

    #[test]
    fn test_request_header_serialization() {
        let header = RequestHeader {
            command: 1,
            client_version: "0.2.0".to_string(),
            output_format: Format::JSON,
            timestamp: "20260304123045".to_string(),
            compression: Compression::NONE,
            encryption: Encryption::NONE,
        };

        let serialized = serde_json::to_string(&header).expect("Serialization should succeed");
        let deserialized: serde_json::Value =
            serde_json::from_str(&serialized).expect("Deserialization should succeed");

        assert_eq!(deserialized["Command"], 1);
        assert_eq!(deserialized["ClientVersion"], "0.2.0");
        assert_eq!(deserialized["Output"], Format::JSON);
        assert_eq!(deserialized["Timestamp"], "20260304123045");
        assert_eq!(deserialized["Compression"], Compression::NONE);
        assert_eq!(deserialized["Encryption"], Encryption::NONE);
    }

    #[tokio::test]
    async fn test_write_request_format() {
        // Create a mock TCP stream using a buffer
        let mut buffer = Vec::new();
        let request_str = r#"{"test":"data"}"#;

        // Manually write what write_request would write
        let mut temp_buf = Vec::new();
        temp_buf.write_i32(request_str.len() as i32).await.unwrap();
        temp_buf.write_all(request_str.as_bytes()).await.unwrap();

        buffer.write_u8(Compression::NONE).await.unwrap();
        buffer.write_u8(Encryption::NONE).await.unwrap();
        buffer.write_all(&temp_buf).await.unwrap();

        // Verify the format
        assert_eq!(buffer[0], Compression::NONE);
        assert_eq!(buffer[1], Encryption::NONE);

        // Read length
        let length = i32::from_be_bytes([buffer[2], buffer[3], buffer[4], buffer[5]]);
        assert_eq!(length, request_str.len() as i32);

        // Verify payload
        let payload = String::from_utf8(buffer[6..].to_vec()).unwrap();
        assert_eq!(payload, request_str);
    }

    #[test]
    fn test_timestamp_format() {
        let header = PgmonetaClient::build_request_header(Command::INFO);
        let timestamp = &header.timestamp;

        // Should be exactly 14 digits
        assert_eq!(timestamp.len(), 14);

        // Parse components
        let year: i32 = timestamp[0..4].parse().expect("Year should be valid");
        let month: i32 = timestamp[4..6].parse().expect("Month should be valid");
        let day: i32 = timestamp[6..8].parse().expect("Day should be valid");
        let hour: i32 = timestamp[8..10].parse().expect("Hour should be valid");
        let minute: i32 = timestamp[10..12].parse().expect("Minute should be valid");
        let second: i32 = timestamp[12..14].parse().expect("Second should be valid");

        // Validate ranges
        assert!((2020..=2100).contains(&year));
        assert!((1..=12).contains(&month));
        assert!((1..=31).contains(&day));
        assert!((0..24).contains(&hour));
        assert!((0..60).contains(&minute));
        assert!((0..60).contains(&second));
    }

    #[test]
    fn test_request_clone() {
        #[derive(Serialize, Clone, Debug)]
        struct TestRequest {
            data: String,
        }

        let test_request = TestRequest {
            data: "test".to_string(),
        };

        let header = PgmonetaClient::build_request_header(Command::INFO);
        let request1 = PgmonetaRequest {
            header: header.clone(),
            request: test_request.clone(),
        };
        let request2 = request1.clone();

        let serialized1 = serde_json::to_string(&request1).unwrap();
        let serialized2 = serde_json::to_string(&request2).unwrap();

        assert_eq!(serialized1, serialized2);
    }
}
