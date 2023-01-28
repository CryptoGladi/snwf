use super::{assert_rsync, RSyncError, DEFAULT_BLOCK_SIZE, DEFAULT_CRYPTO_HASH_SIZE};
use crate::common::DEFAULT_BUFFER_SIZE_FOR_NETWORK;
use crate::prelude::{CoreRecipient, CoreSender, Recipient, Sender};
use crate::protocol::error::ProtocolError;
use crate::protocol::handshake::{send_handshake_from_file, recv_handshake_from_address};
use async_trait::async_trait;
use fast_rsync::SignatureOptions;
use log::debug;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_udt::UdtListener;

#[async_trait(?Send)]
pub trait RSyncRecipient<'a>: CoreRecipient<'a> {
    async fn rsync_sync_file<P>(&mut self, path: P) -> Result<(), RSyncError>
    where
        P: AsRef<Path> + Send + Copy + Sync;
}

#[async_trait(?Send)]
impl<'a> RSyncRecipient<'a> for Recipient<'a> {
    async fn rsync_sync_file<P>(&mut self, path: P) -> Result<(), RSyncError>
    where
        P: AsRef<Path> + Send + Copy + Sync,
    {
        assert_rsync!(path.as_ref().is_file(), "path isn't file or not exists");

        let config = self.get_config();
        debug!(
            "run rsync_sync_file for Sender! config: {:?}, path: {:?}",
            config,
            path.as_ref()
        );

        let mut storage = vec![0u8; DEFAULT_BUFFER_SIZE_FOR_NETWORK];
        let mut buf = vec![0u8; DEFAULT_BUFFER_SIZE_FOR_NETWORK];
        let mut file = File::open(path)
            .await
            .map_err(|e| RSyncError::Protocol(ProtocolError::FileIO(e)))?;

        let mut udt_listener =
            UdtListener::bind((config.addr, config.port_for_handshake).into(), None)
                .await
                .map_err(|e| RSyncError::Protocol(ProtocolError::Bind(e)))?;
        let mut tcp_listener = TcpListener::bind((config.addr, config.port_for_handshake))
            .await
            .map_err(|e| RSyncError::Protocol(ProtocolError::Bind(e)))?;

        recv_handshake_from_address(&mut tcp_listener)
            .await
            .map_err(|e| RSyncError::Protocol(ProtocolError::Handshake(e)))?;

        let (mut tcp_socket, addr) = tcp_listener.accept().await.unwrap();
        loop {
            let len = file
                .read_buf(&mut buf)
                .await
                .map_err(|e| RSyncError::Protocol(ProtocolError::FileIO(e)))?;

            if len == 0 {
                break;
            }

            fast_rsync::Signature::calculate(
                &buf[..],
                &mut storage,
                SignatureOptions {
                    block_size: DEFAULT_BLOCK_SIZE,
                    crypto_hash_size: DEFAULT_CRYPTO_HASH_SIZE,
                },
            );

            tcp_socket.write_all(&mut storage).await.unwrap();
        }

        Ok(())
    }
}
