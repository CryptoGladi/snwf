use crate::common::{get_hasher, timeout, TIMEOUT};
use log::debug;
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    time::timeout,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct Handshake {
    pub file_hash: String,
}

#[derive(Debug, Error)]
pub enum HandshakeError {
    #[error("serialize or deserialize error")]
    SerdeJson(#[from] serde_json::Error),

    #[error("IO/socket error")]
    IO(#[from] std::io::Error),

    #[error("timeout expired")]
    TimeoutExpired,
}

pub(crate) async fn send_handshake_from_file<P>(
    path: P,
    socket: &mut TcpStream,
) -> Result<Handshake, HandshakeError>
where
    P: AsRef<Path> + Sync,
{
    let mut hasher = get_hasher();
    let hash = file_hashing::get_hash_file(path, &mut hasher)?;
    let handshake = Handshake { file_hash: hash };

    let json = serde_json::to_string(&handshake)?;
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

    let mut json = Vec::with_capacity(4096);
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

        pub(crate) async fn send<P: AsRef<Path> + Sync>(
            path_for_send: P,
            socket: &mut TcpStream,
        ) -> Result<(), HandshakeError> {
            send_handshake_from_file(path_for_send, socket).await?;
            Ok(())
        }

        pub(crate) async fn recv(listener: &mut TcpListener) -> Result<Handshake, HandshakeError> {
            Ok(recv_handshake_from_address(listener).await?)
        }
    }

    #[tokio::test]
    async fn test_handshake() {
        crate::init_logger_for_test();

        let (_temp_dir, path_to_file) = file_hashing::fs::extra::generate_random_file(1000);
        let mut hasher = blake2::Blake2b512::new();
        let hash_to_test_file =
            file_hashing::get_hash_file(path_to_file.path(), &mut hasher).unwrap();

        const ADDRESS: &'static str = "127.0.0.1:45254";
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
                file_hash: hash_to_test_file
            }
        );
    }
}
