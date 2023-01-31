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
//!    sender.set_progress_fn(
//!        Some(move |progressing| println!("progress info: {:?}", progressing)
//!    ));
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
//!
//! # What libraries to use
//!
//! * [`tokio-udt`](https://github.com/Distributed-EPFL/tokio-udt) - implementation udt for [tokio](https://tokio.rs/)

mod detail;
pub mod error;
pub mod prelude;
mod raw;

pub mod udt_recipient;
pub mod udt_sender;

#[cfg(test)]
mod tests {
    use crate::{common::get_hasher, core::*, prelude::*};
    use log::debug;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn send_and_recv_udt_with_progress_fn() {
        crate::common::init_logger_for_test();

        let run_progressing_sender_yield = Arc::new(Mutex::new(false));
        let run_progressing_sender_done = Arc::new(Mutex::new(false));
        let run_progressing_recipient_yield = Arc::new(Mutex::new(false));
        let run_progressing_recipient_done = Arc::new(Mutex::new(false));

        let (temp_dir, path_input) = file_hashing::fs::extra::generate_random_file(4352);
        let path_output = temp_dir.join("tess_file.txt");

        let mut sender = Sender::new("127.0.0.1".parse().unwrap(), 4224, 6243);

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
                        path_to_file: _,
                    } => *run_progressing_sender_yield_clone.lock().unwrap() = true,
                    Progressing::Done => *run_progressing_sender_done_clone.lock().unwrap() = true,
                }
            }));
        }

        let mut recipient = Recipient::new("::0".parse().unwrap(), 4224, 6243);

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
                        path_to_file: _,
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

    #[tokio::test]
    async fn send_and_recv_udt_with_original_name() {
        crate::common::init_logger_for_test();

        let (_temp_dir, path_input) = file_hashing::fs::extra::generate_random_file(4352);
        let (output_dir, _path_input) = file_hashing::fs::extra::generate_random_file(1);
        debug!("done generate test files");

        let mut sender = Sender::new("127.0.0.1".parse().unwrap(), 3124, 5143);
        let mut recipient = Recipient::new("::0".parse().unwrap(), 3124, 5143);

        let (recv, send) = tokio::join!(
            recipient.udt_recv_file_with_original_file_name(output_dir.path()),
            sender.udt_send_file(path_input.path())
        );

        send.unwrap();
        recv.unwrap();

        let hash_input = file_hashing::get_hash_file(&path_input, &mut get_hasher()).unwrap();
        let hash_output = file_hashing::get_hash_file(
            output_dir.join(path_input.file_name().unwrap()),
            &mut get_hasher(),
        )
        .unwrap();

        assert_eq!(hash_input, hash_output);
    }
}
