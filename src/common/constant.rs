use blake2::{Blake2b512, Digest};
use std::time::Duration;

/// Default timeout
pub(crate) const DEFAULT_TIMEOUT: Duration = Duration::from_millis(1000);

/// Get default hasher
pub(crate) fn get_hasher() -> Blake2b512 {
    Blake2b512::new()
}
