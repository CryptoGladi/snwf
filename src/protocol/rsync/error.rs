use crate::protocol::error::ProtocolError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RSyncError {
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
            return Err(crate::protocol::rsync::RSyncError::Assert(format!($($message_error)*)));
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
