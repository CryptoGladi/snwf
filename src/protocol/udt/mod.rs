//! [UDT](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) implementation
//!
//! # Example
//! 
//! ```no_run
//! # use snwf::prelude::*;
//! # use std::path::Path;
//! #
//! #[tokio::main]
//! async fn main() {
//!    let mut sender = Sender::new("127.0.0.1".parse().unwrap(), 4324, 6343);
//!    let mut recipient = Recipient::new("::0".parse().unwrap(), 4324, 6343);
//!
//!    sender.set_progress_fn(Some(|progressing| println!("progress info: {:?}", progressing) ));
//!    
//!    let (recv, send) = tokio::join!(
//!        recipient.udt_recv_file(Path::new("other_file.txt")),
//!        sender.udt_send_file(Path::new("file_for_send.txt"))
//!    );
//!    
//!    send.unwrap();
//!    recv.unwrap();
//! }
//! ```
//! 
//! # How it works?
//!
//! 1. We send a handshake that contains the checksum, the
//! name of the original file and the file size
//! 2. Running the udt implementation
//! 
//! And so for **EVERY** file

use crate::common::timeout;
use crate::recipient::{CoreRecipient, Recipient};
use crate::sender::{CoreSender, Sender};
use async_trait::async_trait;
use log::debug;
use std::fmt::Debug;
use std::path::Path;
use tokio::net::TcpListener;
use tokio_udt::UdtListener;

mod detail;
pub mod error;
mod raw;

pub use error::UdtError;

/// [UDT](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) trait for [`CoreSender`]
#[async_trait(?Send)]
pub trait UdtSender<'a>: CoreSender<'a> {
    /// Send file via [udt](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) protocol
    async fn udt_send_file<P>(&mut self, path: P) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync + Debug;

    async fn udt_send_files<P>(&mut self, paths: &Vec<P>) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync + Debug;
}

#[async_trait(?Send)]
impl<'a> UdtSender<'a> for Sender<'a> {
    /// [UDT](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) implementation for [`CoreSender`]
    async fn udt_send_file<P>(&mut self, path: P) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync + Debug,
    {
        let config = self.get_config();
        debug!(
            "running udt_send_file; config: {:?}; path: {:?}",
            config, path
        );

        let (mut udt, mut socket_for_handshake) = detail::connect_for_sender(&config).await?;
        raw::send_file(&mut udt, path, &mut socket_for_handshake, &Some(config), 0).await?;

        Ok(())
    }

    async fn udt_send_files<P>(&mut self, paths: &Vec<P>) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync + Debug,
    {
        let config = self.get_config();
        debug!(
            "running udt_send_file; config: {:?}; paths: {:?}",
            config, paths
        );

        let (mut udt, mut socket_for_handshake) = detail::connect_for_sender(&config).await?;

        for (number_file, path) in paths.iter().enumerate() {
            raw::send_file(
                &mut udt,
                path,
                &mut socket_for_handshake,
                &Some(config.clone()),
                number_file as u64,
            )
            .await?;
        }

        Ok(())
    }
}

/// [UDT](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) trait for [`CoreRecipient`]
#[async_trait(?Send)]
pub trait UdtRecipient<'a>: CoreRecipient<'a> {
    /// Receive a file via [udt](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) protocol
    async fn udt_recv_file<P>(&mut self, output: P) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync;
}

#[async_trait(?Send)]
impl<'a> UdtRecipient<'a> for Recipient<'a> {
    /// [UDT](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) implementation for [`CoreRecipient`]
    async fn udt_recv_file<P>(&mut self, output: P) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync,
    {
        let config = self.get_config();
        debug!("running udt_recv_file; config: {:?}", config);

        let udt_listener =
            UdtListener::bind((config.addr, config.port_for_send_files).into(), None)
                .await
                .map_err(UdtError::Bind)?;
        debug!("done socket udt bind");

        let mut tcp_handshake = TcpListener::bind((config.addr, config.port_for_handshake))
            .await
            .map_err(UdtError::Bind)?;
        debug!("done socket handshake bind");

        let (addr, mut connection) = timeout!(
            udt_listener.accept(),
            |_| UdtError::TimeoutExpired,
            config.timeout
        )?
        .map_err(UdtError::Accept)?;
        debug!("accepted connection from {}", addr);

        raw::recv_file(&mut connection, &mut tcp_handshake, output, &Some(config), 0).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{get_hasher, Progressing};
    use std::{
        cell::RefCell,
        sync::{Arc, Mutex},
    };

    #[tokio::test]
    async fn send_and_recv_udt() {
        crate::init_logger_for_test();

        let run_progressing_sender_yield = Arc::new(Mutex::new(false));
        let run_progressing_sender_done = Arc::new(Mutex::new(false));
        let run_progressing_recipient_yield = Arc::new(Mutex::new(false));
        let run_progressing_recipient_done = Arc::new(Mutex::new(false));

        let (temp_dir, path_input) = file_hashing::fs::extra::generate_random_file(4352);
        let path_output = temp_dir.join("tess_file.txt");

        let mut sender = Sender::new("127.0.0.1".parse().unwrap(), 4324, 6343);

        {
            let run_progressing_sender_yield_clone = run_progressing_sender_yield.clone();
            let run_progressing_sender_done_clone = run_progressing_sender_done.clone();

            sender.set_progress_fn(Some(move |progressing| {
                debug!("progressing sender: {:?}", progressing);

                match progressing {
                    Progressing::Yield {
                        done_files: _,
                        total_bytes: _,
                        done_bytes: _,
                    } => *run_progressing_sender_yield_clone.lock().unwrap() = true,
                    Progressing::Done => *run_progressing_sender_done_clone.lock().unwrap() = true,
                }
            }));
        }

        let mut recipient = Recipient::new("::0".parse().unwrap(), 4324, 6343);

        {
            let run_progressing_recipient_yield_clone = run_progressing_recipient_yield.clone();
            let run_progressing_recipient_done_clone = run_progressing_recipient_done.clone();

            recipient.set_progress_fn(Some(move |progressing| {
                debug!("progressing recipient: {:?}", progressing);

                match progressing {
                    Progressing::Yield {
                        done_files: _,
                        total_bytes: _,
                        done_bytes: _,
                    } => *run_progressing_recipient_yield_clone.lock().unwrap() = true,
                    Progressing::Done => {
                        *run_progressing_recipient_done_clone.lock().unwrap() = true
                    }
                }
            }));
        }

        let (recv, send) = tokio::join!(
            recipient.udt_recv_file(path_output.as_path()),
            sender.udt_send_file(path_input.path())
        );

        send.unwrap();
        recv.unwrap();

        let hash_input = file_hashing::get_hash_file(path_input, &mut get_hasher()).unwrap();
        let hash_output = file_hashing::get_hash_file(path_output, &mut get_hasher()).unwrap();

        assert_eq!(
            (
                hash_input,
                (*run_progressing_sender_yield.lock().unwrap()
                    && *run_progressing_sender_done.lock().unwrap()
                    && *run_progressing_recipient_yield.lock().unwrap()
                    && *run_progressing_recipient_done.lock().unwrap())
            ),
            (hash_output, true)
        );
    }
}
