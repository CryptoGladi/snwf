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
        todo!()
    }
}
