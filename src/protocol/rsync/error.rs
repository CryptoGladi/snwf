//! All error for [`rsync`](crate::protocol::rsync)

use crate::protocol::error::ProtocolError;
use thiserror::Error;

/// Enum error
#[derive(Debug, Error)]
pub enum RSyncError {
    /// Set of standard errors for any protocol
    ///
    /// # Example
    ///
    /// ```
    /// use snwf::prelude::{*, ProtocolError::*};
    ///
    /// let i = RSyncError::Protocol(TimeoutExpired);
    /// ```
    #[error("problem in protocol: {0}")]
    Protocol(ProtocolError),

    /// Wrong use function in [`rsync`](crate::protocol::rsync)
    #[error("wrong use function: {0}")]
    Assert(String),
}

/// [`std::assert`], but for [`RSyncError`]
///
/// # Example
///
/// See unit tests
macro_rules! assert_rsync {
    ($for_check:expr, $($message_error:tt)*) => {
        if $for_check == false {
            log::error!("assert rsync! message_error: {}", format!($($message_error)*));
            return Err(crate::protocol::rsync::error::RSyncError::Assert(format!($($message_error)*)));
        }
    };
}

pub(crate) use assert_rsync;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_assert_rsync() {
        let fn_test = || -> Result<(), RSyncError> {
            assert_rsync!(false, "test message: {}", "test value");
            Ok(())
        };

        match fn_test().err().unwrap() {
            RSyncError::Assert(message) => assert_eq!(message, "test message: test value"),
            _ => panic!("fn_test() != UdtError::Assert"),
        }
    }
}
