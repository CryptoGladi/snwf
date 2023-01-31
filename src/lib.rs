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
//! * **udt** - [udt](crate::protocol::udt) protocol
//! * **rsync** - [rsync](crate::protocol::rsync) for sync files
//! * [Callback function](crate::core::Progressing)
//! * Use `#![forbid(unsafe_code)]`
//!
//! # Example
//!
//! Documentation is in [`protocol`]
//!
//! ```no_run
//! use snwf::prelude::*;
//! use std::path::Path;
//! use std::sync::{Mutex, Arc};
//!
//! #[tokio::main]
//! async fn main() {
//!    let mut sender = Sender::new("127.0.0.1".parse().unwrap(), 4324, 6343);
//!    let mut recipient = Recipient::new("::0".parse().unwrap(), 4324, 6343);
//!
//!    let random_variable = Arc::new(Mutex::new(false));
//!
//!    {
//!    let random_variable_clone = random_variable.clone();
//!
//!    sender.set_progress_fn(Some(move |progressing|
//!         {
//!             println!("progress info: {:?}", progressing);
//!             *random_variable_clone.lock().unwrap() = true;
//!         }));
//!    }
//!    
//!    let (recv, send) = tokio::join!(
//!        recipient.udt_recv_file(Path::new("other_file.txt")),
//!        sender.udt_send_file(Path::new("file_for_send.txt"))
//!    );
//!    
//!    send.unwrap();
//!    recv.unwrap();
//!
//!    assert_eq!(*random_variable.lock().unwrap(), true);
//! }
//! ```
//!
//! * [`Sender`](crate::sender::Sender) - only send files
//! * [`Recipient`](crate::recipient::Recipient) - only receives files
//!
//! snwf by [CryptoGladi](https://github.com/CryptoGladi)

#![forbid(unsafe_code)]
#![warn(unused_lifetimes)]
#![warn(clippy::disallowed_types)]
#![warn(clippy::unused_async)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::use_self)]
#![warn(clippy::unreadable_literal)]
#![warn(clippy::unreachable)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unnested_or_patterns)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::too_many_lines)]
#![warn(clippy::todo)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::macro_use_imports)]
#![warn(clippy::inline_always)]
#![warn(clippy::cast_possible_truncation)]

pub mod common;
pub mod core;
pub mod prelude;
pub mod protocol;
pub mod recipient; // or client
pub mod sender; // or server