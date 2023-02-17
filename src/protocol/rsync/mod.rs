//! Sync files by rsync
//!
//! # How it works?
//!
//! ```text
//! ┌─────────────────┐         ┌────────────────┐
//! │                 │    1    │                │
//! │    Recipient    │◄───────►│     Sender     │
//! │                 │         │                │
//! └─────────────────┴──┐    ┌─┴────────────────┘
//!                      ▼    ▼
//!                    ┌────────┐
//!                    │        │
//!                 2) │  Loop  │
//!                    │        │
//!           ┌────────┴────────┴────────┐
//!           ▼                          ▼
//!  ┌─────────────────┐         ┌────────────────┐
//!  │                 │    3    │                │
//!  │    Recipient    ├────────►│     Sender     │
//!  │                 │         │                │
//!  └─────────────────┘         └────────────────┘
//!           ▼             ▼             ▼
//!  ┌─────────────────┐         ┌────────────────┐
//!  │                 │    4    │                │
//!  │    Recipient    │◄────────┤     Sender     │
//!  │                 │         │                │
//!  └─────────────────┴──┐    ┌─┴────────────────┘
//!                       ▼    ▼
//!                      ┌──────┐
//!                   5) │ Done │
//!                      └──────┘
//! ```
//!
//! 1. Send and recv handshake
//! 2. All sent actions will be performed in a loop. This is done in order
//!    to do all the work in small parts and not load the RAM.
//! 3. Send `Signature`. This is cheap because the signature can be
//!    1000X smaller than `foo_recipient` itself.
//! 4. `Sender` calculates a diff from `Recipient` and send to `Recipient`.
//! 5. End loop. `Recipient` attempts to apply the `delta` to `foo_recipient`. The
//!    resulting data is probably (*) equal to foo_sender.
//!
//! # What libraries to use
//!
//! * [`tokio-udt`](https://github.com/Distributed-EPFL/tokio-udt) - implementation udt for [tokio](https://tokio.rs/)
//! * [`fast-rsync`](https://github.com/dropbox/fast_rsync) - an optimized implementation of
//!   [librsync](https://github.com/librsync/librsync) in pure Rust

pub mod error;
pub mod prelude;
mod raw;
pub mod rsync_recipient;
pub mod rsync_sender;
pub(crate) mod constant;