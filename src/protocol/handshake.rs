//! Handshake - send information about a file
//!
//! # Description
//!
//! Handshake - used information about a file for check valid.
//!
//! * Format: [json](https://github.com/serde-rs/json)
//! * Max size: 512 (hash) + 300 (filename) + 60 (other information) = 872
//!
//! **The algorithm of work may differ from the type of [`crate::protocol`]!**

use crate::common::{get_hasher, timeout, DEFAULT_BUFFER_SIZE_FOR_NETWORK};
use log::debug;
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;
use tokio::{
    fs::metadata,
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

/// Info about file
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct Handshake {
    pub(crate) hash: String,
    pub(crate) size: u64,
    pub(crate) file_name: String,
}

#[derive(Debug, Error)]
pub enum HandshakeError {
    #[error("serialize or deserialize error")]
    SerdeJson(#[from] serde_json::Error),

    #[error("IO/socket error")]
    IO(#[from] std::io::Error),

    #[error("timeout expired")]
    TimeoutExpired,

    #[error("wrong use function: {0}")]
    Assert(String),
}

/// [`std::assert`], but for [`HandshakeError`]
///
/// # Example
///
/// See unit tests
macro_rules! assert_handshake {
    ($for_check:expr, $($message_error:tt)*) => {
        if $for_check == false {
            log::error!("assert handshake! message_error: {}", format!($($message_error)*));
            return Err(crate::protocol::handshake::HandshakeError::Assert(format!($($message_error)*)));
        }
    };
}

pub(crate) use assert_handshake;

#[allow(clippy::unwrap_used)]
fn get_file_name_from_as_ref_path(path: impl AsRef<Path>) -> String {
    path.as_ref()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub(crate) async fn send_handshake_from_file<P>(
    path: P,
    socket: &mut TcpStream,
) -> Result<Handshake, HandshakeError>
where
    P: AsRef<Path> + Sync + Copy,
{
    assert_handshake!(path.as_ref().is_file(), "path must be a file");

    let mut hasher = get_hasher();
    let hash = file_hashing::get_hash_file(path, &mut hasher)?;
    let metadata = metadata(path).await?;
    let handshake = Handshake {
        hash,
        size: metadata.len(),
        file_name: get_file_name_from_as_ref_path(path),
    };

    let json = serde_json::to_string(&handshake)?;

    // json >= DEFAULT_BUFFER_SIZE_FOR_NETWORK - is error
    assert_handshake!(
        DEFAULT_BUFFER_SIZE_FOR_NETWORK.cmp(&json.len()).is_ge(),
        "Buffer overflow. json size: {}",
        json.len()
    );

    timeout!(socket.write_all(json.to_string().as_bytes()), |_| {
        HandshakeError::TimeoutExpired
    })??;
    debug!("Done socket 'Handshake' send. Handshake: {:?}", json);

    Ok(handshake)
}

pub(crate) async fn recv_handshake_from_address(
    listener: &mut TcpListener,
) -> Result<Handshake, HandshakeError> {
    let (mut client, addr) = timeout!(listener.accept(), |_| HandshakeError::TimeoutExpired)??;
    debug!("Client for recv handshake: addr {}", addr);

    let mut json = Vec::with_capacity(DEFAULT_BUFFER_SIZE_FOR_NETWORK);
    timeout!(client.read_buf(&mut json), |_| {
        HandshakeError::TimeoutExpired
    })??;

    Ok(serde_json::from_str(&String::from_utf8_lossy(&json[..]))?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use blake2::Digest;

    pub(crate) mod detail {
        use super::*;

        pub(crate) async fn send<P: AsRef<Path> + Sync + Copy>(
            path_for_send: P,
            socket: &mut TcpStream,
        ) -> Result<(), HandshakeError> {
            send_handshake_from_file(path_for_send, socket).await?;
            Ok(())
        }

        pub(crate) async fn recv(listener: &mut TcpListener) -> Result<Handshake, HandshakeError> {
            recv_handshake_from_address(listener).await
        }
    }

    #[tokio::test]
    async fn send_and_recv_handshake() {
        crate::init_logger_for_test();

        let (_temp_dir, path_to_file) = file_hashing::fs::extra::generate_random_file(1000);
        let mut hasher = blake2::Blake2b512::new();
        let hash_from_test_file =
            file_hashing::get_hash_file(path_to_file.path(), &mut hasher).unwrap();

        const ADDRESS: &str = "127.0.0.1:45254";
        let mut recv_socket = TcpListener::bind(ADDRESS).await.unwrap();
        let mut send_socket = TcpStream::connect(ADDRESS).await.unwrap();

        let recv_future = detail::recv(&mut recv_socket);
        let send_future = detail::send(path_to_file.path(), &mut send_socket);

        let (recv, send) = tokio::join!(send_future, recv_future);
        recv.unwrap();

        let handshake = send.unwrap();
        assert_eq!(
            handshake,
            Handshake {
                hash: hash_from_test_file,
                size: 1000,
                file_name: get_file_name_from_as_ref_path(path_to_file)
            }
        );
    }

    #[test]
    fn macro_assert_handshake() {
        let fn_test = || -> Result<(), HandshakeError> {
            assert_handshake!(false, "test message: {}", "test value");
            Ok(())
        };

        match fn_test().err().unwrap() {
            HandshakeError::Assert(message) => assert_eq!(message, "test message: test value"),
            _ => panic!("fn_test() != UdtError::Assert"),
        }
    }
}
