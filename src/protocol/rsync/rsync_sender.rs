use super::prelude::*;
use crate::prelude::{CoreSender, Sender};
use async_trait::async_trait;
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

        Ok(())
    }
}
