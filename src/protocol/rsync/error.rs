use thiserror::Error;

#[derive(Debug, Error)]
pub enum RSyncError {
    /// Wrong use function in [`rsync`](crate::protocol::rsync)
    #[error("wrong use function: {0}")]
    Assert(String),

    /// This error occurs when the files are not working properly
    ///
    /// For example: `You do not have permission to write to the file`
    #[error("IO filesystem")]
    FileIO(#[source] std::io::Error),
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
