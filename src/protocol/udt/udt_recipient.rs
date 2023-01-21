//! Implementation [udt](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) for trait [`CoreRecipient`]

use super::UdtError;
use crate::{
    common::timeout,
    prelude::*,
    protocol::{
        error::ProtocolError,
        handshake::recv_handshake_from_address,
        udt::{detail, error::assert_udt, raw},
    },
};
use async_trait::async_trait;
use log::debug;
use std::path::Path;

/// [UDT](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) trait for [`CoreRecipient`]
#[async_trait(?Send)]
pub trait UdtRecipient<'a>: CoreRecipient<'a> {
    /// Receive a file via [udt](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) protocol
    ///
    /// # Arguments
    ///
    /// * `output` - path to save file.
    ///
    /// # Example
    /// ```no_run
    /// # use snwf::prelude::*;
    /// # use std::path::Path;
    /// #
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut recipient = Recipient::new("::0".parse().unwrap(), 4324, 6343);
    ///
    ///     recipient.udt_recv_file(Path::new("file.txt"));
    /// }
    /// ```
    ///
    /// **Warning:** not save original file name! If we want save it,
    /// use [`UdtRecipient::udt_recv_file_with_original_file_name`]
    async fn udt_recv_file<P>(&mut self, output: P) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync;

    /// Receive a file via [udt](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) protocol
    ///
    /// **But save original name** (not save [`UdtRecipient::udt_recv_file`])
    ///
    /// # Arguments
    ///
    /// * `output` - path to save file.
    ///
    /// # Example
    /// ```no_run
    /// # use snwf::prelude::*;
    /// # use std::path::Path;
    /// #
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut recipient = Recipient::new("::0".parse().unwrap(), 4324, 6343);
    ///
    ///     recipient.udt_recv_file_with_original_file_name(Path::new("/home/gladi/Downloads"));
    /// }
    /// ```
    async fn udt_recv_file_with_original_file_name<P>(&mut self, output: P) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync;
}

#[async_trait(?Send)]
impl<'a> UdtRecipient<'a> for Recipient<'a> {
    async fn udt_recv_file<P>(&mut self, output: P) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync,
    {
        assert_udt!(
            !output.as_ref().exists(),
            "output must be no exists. output path: {}",
            output.as_ref().display()
        );

        let config = self.get_config();
        debug!("running udt_recv_file; config: {:?}", config);

        let (udt_listener, mut tcp_handshake) = detail::all_bind_for_recipient(&config).await?;

        let (addr, mut connection) = timeout!(
            udt_listener.accept(),
            |_| UdtError::Protocol(ProtocolError::TimeoutExpired),
            config.timeout
        )?
        .map_err(|e| UdtError::Protocol(ProtocolError::Accept(e)))?;
        debug!("accepted connection from {}", addr);

        raw::recv_file(
            &mut connection,
            &mut tcp_handshake,
            output,
            &Some(config),
            0,
            None,
        )
        .await?;

        Ok(())
    }

    async fn udt_recv_file_with_original_file_name<P>(&mut self, output: P) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync,
    {
        assert_udt!(output.as_ref().is_dir(), "output must be a folder path");

        let config = self.get_config();
        debug!("running udt_recv_file; config: {:?}", config);

        let (udt_listener, mut tcp_handshake) = detail::all_bind_for_recipient(&config).await?;

        let (addr, mut connection) = timeout!(
            udt_listener.accept(),
            |_| UdtError::Protocol(ProtocolError::TimeoutExpired),
            config.timeout
        )?
        .map_err(|e| UdtError::Protocol(ProtocolError::Accept(e)))?;
        debug!("accepted connection from {}", addr);

        let handshake = recv_handshake_from_address(&mut tcp_handshake)
            .await
            .map_err(|e| UdtError::Protocol(ProtocolError::Handshake(e)))?;
        raw::recv_file(
            &mut connection,
            &mut tcp_handshake,
            Path::new(&output.as_ref().join(handshake.file_name.clone())),
            &Some(config),
            0,
            Some(handshake),
        )
        .await?;

        Ok(())
    }
}
