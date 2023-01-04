//! # snwf
//!
//! Library for simple network work on files
//!
//! # Motivation
//!
//! If you just need to transfer a file over the network to another computer,
//! but you don't want to write hundreds of lines of code to implement a
//! "receiver" and "sender", then this library is right for you.
//!
//! # Features
//!
//! * **udt** - [udt](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) protocol
//! * [Callback function](crate::common::alias::Progressing)
//! * Use `#![forbid(unsafe_code)]`
//!
//! # Example
//!
//! Documentation is in [`protocol`]
//!
//! ```no_run
//! use snwf::prelude::*;
//! use std::path::Path;
//!
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
//! * [`sender::Sender`] - only send files
//! * [`recipient::Recipient`] - Only receives files
//! 
//! snwf by [CryptoGladi](https://github.com/CryptoGladi)

#![forbid(unsafe_code)]

pub mod common;
pub mod prelude;
pub mod protocol;
pub mod recipient; // or client
pub mod sender; // or server

/// Init logger for **tests**
#[cfg(test)]
pub fn init_logger_for_test() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(log::LevelFilter::Warn)
        .filter_module("snwf", log::LevelFilter::Debug)
        .try_init();
}
