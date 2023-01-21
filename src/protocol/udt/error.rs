//! All error in [`udt`](crate::protocol::udt)

use crate::protocol::error::ProtocolError;
use thiserror::Error;

/// Enum error
#[derive(Debug, Error)]
pub enum UdtError {
    #[error("problem in protocol: {0}")]
    Protocol(ProtocolError),

    /// Wrong use function in [udt](crate::protocol::udt)
    #[error("wrong use function: {0}")]
    Assert(String),
}

/// [`std::assert`], but for [`UdtError`]
///
/// # Example
///
/// See unit tests
macro_rules! assert_udt {
    ($for_check:expr, $($message_error:tt)*) => {
        if $for_check == false {
            log::error!("assert udt! message_error: {}", format!($($message_error)*));
            return Err(crate::protocol::udt::UdtError::Assert(format!($($message_error)*)));
        }
    };
}

pub(crate) use assert_udt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_assert_udt() {
        let fn_test = || -> Result<(), UdtError> {
            assert_udt!(false, "test message: {}", "test value");
            Ok(())
        };

        match fn_test().err().unwrap() {
            UdtError::Assert(message) => assert_eq!(message, "test message: test value"),
            _ => panic!("fn_test() != UdtError::Assert"),
        }
    }
}
