//! Useful constant or const fn

use blake2::{Blake2b512, Digest};
use std::time::Duration;

/// Default timeout
pub(crate) const DEFAULT_TIMEOUT: Duration = Duration::from_millis(1000);

///  Default buffer size for **network**
pub(crate) const DEFAULT_BUFFER_SIZE_FOR_NETWORK: usize = 4_096; // TODO

///  Default buffer size for **file**
pub(crate) const DEFAULT_BUFFER_SIZE_FOR_FILE: usize = 4_096;

/// Get default hasher
pub(crate) fn get_hasher() -> Blake2b512 {
    Blake2b512::new()
}
