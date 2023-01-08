use super::RSyncError;
use crate::prelude::{CoreRecipient, Recipient};
use async_trait::async_trait;
use std::path::Path;

#[async_trait(?Send)]
pub trait RSyncRecipient<'a>: CoreRecipient<'a> {
    async fn rsync_sync_file<P>(&mut self, output: P) -> Result<(), RSyncError>
    where
        P: AsRef<Path> + Send + Copy + Sync;
}

#[async_trait(?Send)]
impl<'a> RSyncRecipient<'a> for Recipient<'a> {
    async fn rsync_sync_file<P>(&mut self, output: P) -> Result<(), RSyncError>
    where
        P: AsRef<Path> + Send + Copy + Sync,
    {
        // TODO

        Ok(())
    }
}
