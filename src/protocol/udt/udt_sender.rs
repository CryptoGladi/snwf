//! Implementation [udt](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) for trait [`CoreSender`]

use super::UdtError;
use crate::{
    prelude::*,
    protocol::udt::{detail, error::assert_udt, raw},
};
use async_trait::async_trait;
use log::debug;
use std::fmt::Debug;
use std::path::Path;

/// [UDT](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) trait for [`CoreSender`]
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
    async fn udt_send_file<P>(&mut self, path: P) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync + Debug;
}

#[async_trait(?Send)]
impl<'a> UdtSender<'a> for Sender<'a> {
    async fn udt_send_file<P>(&mut self, path: P) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync + Debug,
    {
        assert_udt!(path.as_ref().is_file(), "path isn't file or not exists");
        let config = self.get_config();

        debug!(
            "running udt_send_file; config: {:?}; path: {:?}",
            config, path
        );

        let (mut udt, mut socket_for_handshake) = detail::all_connect_for_sender(&config).await?;
        raw::send_file(&mut udt, path, &mut socket_for_handshake, &Some(config), 0).await?;

        Ok(())
    }
}
