use blake2::{Blake2b512, Digest};
use std::time::Duration;

pub(crate) const TIMEOUT: Duration = Duration::from_millis(1000);

macro_rules! timeout {
    ($x:expr, $e:expr) => {
        timeout(crate::common::TIMEOUT, $x).await.map_err($e)
    };
}

pub(crate) use timeout;

pub(crate) fn get_hasher() -> Blake2b512 {
    Blake2b512::new()
}
