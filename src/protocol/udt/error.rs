use crate::protocol::handshake::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UdtError {
    #[error("bind socket")]
    Bind(#[source] std::io::Error),

    #[error("accept sender")]
    Accept(#[source] std::io::Error),

    #[error("connection to recipient")]
    Connect(#[source] std::io::Error),

    #[error("IO filesystem")]
    FileIO(#[source] std::io::Error),

    #[error("IO network")]
    NetworkIO(#[source] std::io::Error),

    #[error("handshake")]
    Handshake(#[from] HandshakeError),

    #[error("file invalid. Check network")]
    FileInvalid,

    #[error("timeout expired")]
    TimeoutExpired,
}
