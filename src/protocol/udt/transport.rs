use super::UdtError;
use crate::{core::Transport, prelude::CoreSender};
use async_trait::async_trait;
use std::path::Path;

pub struct UdtTransport {}

#[async_trait(?Send)]
impl Transport<UdtError> for UdtTransport {
    async fn recv_file<P>(&mut self, output: P) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync,
    {
        Ok(())
    }
}
