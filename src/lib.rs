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
//! 'udt' - enable [udt](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) protocol support

pub(crate) mod common;
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
