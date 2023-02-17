use super::prelude::*;
use crate::protocol::error::ProtocolError::*;
use crate::{
    prelude::{CoreSender, Sender},
    protocol::{handshake::send_handshake_from_file, rsync::raw},
};
use async_trait::async_trait;
use log::debug;
use std::path::Path;

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

        let (mut udt_connection, mut tcp_connection) = raw::connect_all(&config).await?;
        send_handshake_from_file(path, &mut tcp_connection)
            .await
            .map_err(|e| RSyncError::Protocol(Handshake(e)))?;

        let signature = raw::get_big_message(&udt_connection);

        Ok(())
    }
}
