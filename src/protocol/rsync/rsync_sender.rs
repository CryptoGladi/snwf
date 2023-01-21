use super::{assert_rsync, RSyncError, DEFAULT_BLOCK_SIZE, DEFAULT_CRYPTO_HASH_SIZE};
use crate::common::DEFAULT_BUFFER_SIZE_FOR_NETWORK;
use crate::prelude::{CoreSender, Sender};
use crate::protocol::error::ProtocolError;
use crate::protocol::handshake::send_handshake_from_file;
use async_trait::async_trait;
use fast_rsync::SignatureOptions;
use log::debug;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[async_trait(?Send)]
pub trait RSyncSender<'a>: CoreSender<'a> {
    async fn rsync_sync_file<P>(&mut self, path: P) -> Result<(), RSyncError>
    where
        P: AsRef<Path> + Send + Copy + Sync;
}

#[async_trait(?Send)]
impl<'a> RSyncSender<'a> for Sender<'a> {
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

        let mut tcp_socket = TcpStream::connect((config.addr, config.port_for_handshake))
            .await
            .map_err(|e| RSyncError::Protocol(ProtocolError::Connect(e)))?;
        send_handshake_from_file(path, &mut tcp_socket)
            .await
            .map_err(|e| RSyncError::Protocol(ProtocolError::Handshake(e)))?;

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
        }

        Ok(())
    }
}
