use blake2::{Blake2b512, Digest};
use std::time::Duration;

pub(crate) const TIMEOUT: Duration = Duration::from_millis(1000);

pub(crate) fn get_hasher() -> Blake2b512 {
    Blake2b512::new()
}
