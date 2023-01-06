use super::RSyncError;
use crate::{core::Transport, prelude::CoreSender};
use async_trait::async_trait;
use std::path::Path;

#[async_trait(?Send)]
pub trait UdtSender<'a>: CoreSender<'a> {
    /// Send file via [udt](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) protocol
    ///
    /// # Example
    /// ```no_run
    /// # use snwf::prelude::*;
    /// # use std::path::Path;
    /// #
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut sender = Sender::new("127.0.0.1".parse().unwrap(), 4324, 6343);
    ///
    ///     sender.udt_send_file(Path::new("file.txt"));
    /// }
    async fn rsync_sync_file<P, TransportError>(
        &mut self,
        path: P,
        transport: impl Transport<TransportError>,
    ) -> Result<(), RSyncError>
    where
        P: AsRef<Path> + Send + Copy + Sync;
}
