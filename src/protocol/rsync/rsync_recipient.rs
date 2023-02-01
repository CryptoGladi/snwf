use super::prelude::*;
use crate::prelude::{CoreRecipient, Recipient};
use crate::protocol::error::ProtocolError::*;
use crate::protocol::handshake::recv_handshake_from_address;
use crate::protocol::rsync::raw;
use async_trait::async_trait;
use log::debug;
use std::path::Path;

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
            "run rsync_sync_file for Recipient! config: {:?}, path: {:?}",
            config,
            path.as_ref()
        );

        let (udt_listener, tcp_listener) = raw::bind_all(&config).await?;

        recv_handshake_from_address(&mut tcp_listener)
            .await
            .map_err(|e| RSyncError::Protocol(Handshake(e)))?;

        let (addr, udt_connection) = udt_listener
            .accept()
            .await
            .map_err(|e| RSyncError::Protocol(Accept(e)))?;
        debug!("new accept! addr: {addr}");

        raw::send_signature(path, &udt_connection).await?;

        Ok(())
    }
}
