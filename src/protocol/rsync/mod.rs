//! Sync files by rsync
//!
//! # How it works?
//!
//! 1. Host B calculates the `Signature` of `foo_B` and sends it to `A`. This is
//!    cheap because the signature can be 1000X smaller than `foo_B` itself.
//!    (The precise factor is configurable and creates a tradeoff between
//!    signature size and usefulness. A larger signature enables the creation
//!    of smaller and more precise deltas.)
//! 2. `Host A` calculates a diff from `B's` signature and `foo_A`, and sends it to `B`.
//! 3. `Host B` attempts to apply the `delta` to `foo_B`. The resulting data
//!    is probably (*) equal to foo_A.
//!
//! # What libraries to use
//!
//! * [`tokio-udt`](https://github.com/Distributed-EPFL/tokio-udt) - implementation udt for [tokio](https://tokio.rs/)
//! * [`fast-rsync`](https://github.com/dropbox/fast_rsync) - an optimized implementation of
//!   [librsync](https://github.com/librsync/librsync) in pure Rust

pub mod constant;
pub mod error;
mod raw;
pub mod rsync_recipient;
pub mod rsync_sender;

pub use constant::*;
pub(crate) use error::assert_rsync;
pub use error::RSyncError;
pub use rsync_recipient::RSyncRecipient;
pub use rsync_sender::RSyncSender;
