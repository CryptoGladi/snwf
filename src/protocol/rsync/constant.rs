use crate::common::DEFAULT_BUFFER_SIZE_FOR_NETWORK;
use fast_rsync::SignatureOptions;

pub(crate) const DEFAULT_BLOCK_SIZE: u32 = DEFAULT_BUFFER_SIZE_FOR_NETWORK as u32;

pub(crate) const DEFAULT_CRYPTO_HASH_SIZE: u32 = 256;

pub(crate) const SIGNATURE_OPTIONS: SignatureOptions = SignatureOptions {
    block_size: DEFAULT_BLOCK_SIZE,
    crypto_hash_size: DEFAULT_CRYPTO_HASH_SIZE,
};

/// Symbol for end-of-transmission
///
/// It is [EOT](https://en.wikipedia.org/wiki/End-of-Transmission_character). [Value](http://www.csc.villanova.edu/~tway/resources/ascii-table.html)
pub(crate) const STOP_WORD: &[u8] = &[0x04];
