//! Module for **core** object

use crate::common::Progressing;
use async_trait::async_trait;
use std::{net::IpAddr, path::Path, time::Duration};

/// Trait for config
///
/// use [`crate::sender::ConfigSender`] and [`crate::recipient::ConfigRecipient`]
pub trait CoreConfig {
    /// Get IP address for bind or connect
    fn get_addr(&self) -> IpAddr;

    /// Get port for sending files. Uses this port only [`crate::protocol`]
    fn get_port_for_send_files(&self) -> u16;

    /// Get handshake port. The [`crate::protocol`] does not use it
    fn get_port_for_handshake(&self) -> u16;

    /// Get timeout for getting error
    fn get_timeout(&self) -> Duration;

    /// Run callback
    ///
    /// Callback to check the progress of the operation
    fn run_progress_fn(&self, progressing: Progressing);
}

#[async_trait]
pub trait Transport<Error> {
    async fn recv_file<P>(&mut self, output: P) -> Result<(), Error>
    where
        P: AsRef<Path> + Send + Copy + Sync;
}
