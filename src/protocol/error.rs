//! Errors for any protocol

use super::handshake::HandshakeError;
use thiserror::Error;

/// All error options that can be in any protocol
#[derive(Debug, Error)]
pub enum ProtocolError {
    /// An error occurred while enabling the server socket (TcpListener)
    ///
    /// # Example
    ///
    /// ```should_panic
    /// # use std::net::TcpListener;
    /// #
    /// let server_socket = TcpListener::bind("127.0.0.1:-1").unwrap();
    /// ```
    #[error("bind socket")]
    Bind(#[source] std::io::Error),

    /// The error occurs when the socket is not accepted correctly
    ///
    /// Please, see [it](std::net::TcpListener::accept)
    ///
    /// **Do not confuse with [`ProtocolError::TimeoutExpired`]!**
    #[error("accept socket")]
    Accept(#[source] std::io::Error),

    /// This error occurs when the connection is wrong
    ///
    /// Please, see [it](std::net::TcpStream::connect)
    ///
    /// **Do not confuse with [`ProtocolError::TimeoutExpired`]!**
    #[error("connection to socket")]
    Connect(#[source] std::io::Error),

    /// This error occurs when the files are not working properly
    ///
    /// For example: `You do not have permission to write to the file`
    #[error("IO filesystem")]
    IO(#[source] std::io::Error),

    /// This error occurs if we cannot receive something from the socket
    ///
    /// Please, see [it](std::io::Read::read) and [it](std::net::TcpStream)
    #[error("receiving data")]
    ReceivingData(#[source] std::io::Error),

    /// Handshake error
    #[error("handshake")]
    Handshake(#[from] HandshakeError),

    /// Checksum is different
    #[error("file invalid")]
    FileInvalid,

    /// Each operation that is connected to the network has time limit
    ///
    /// This is the timeout
    #[error("timeout expired")]
    TimeoutExpired,
}
