use super::{assert_rsync, RSyncError, DEFAULT_BLOCK_SIZE, DEFAULT_CRYPTO_HASH_SIZE};
use crate::common::DEFAULT_BUFFER_SIZE_FOR_NETWORK;
use crate::prelude::{CoreRecipient, Recipient};
use crate::protocol::error::ProtocolError;
use crate::protocol::handshake::recv_handshake_from_address;
use crate::protocol::rsync::raw;
use async_trait::async_trait;
use fast_rsync::SignatureOptions;
use log::debug;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

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

        let (udt_listener, mut tcp_listener) = raw::bind_all(&config).await?;

        recv_handshake_from_address(&mut tcp_listener)
            .await
            .map_err(|e| RSyncError::Protocol(ProtocolError::Handshake(e)))?;

        let (addr, udt_connection) = udt_listener.accept().await.unwrap();
        debug!("new accept! addr: {addr}");

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

            udt_connection.send(&storage).await.unwrap();
        }

        Ok(())
    }
}
