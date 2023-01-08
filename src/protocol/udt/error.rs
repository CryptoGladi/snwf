//! All error in [`udt`](crate::protocol::udt)

use crate::protocol::handshake::*;
use thiserror::Error;

/// Enum error
#[derive(Debug, Error)]
pub enum UdtError {
    /// An error occurred while enabling the server socket (TcpListener)
    ///
    /// # Example
    ///
    /// ```should_panic
    /// # use std::net::TcpListener;
    /// #
    /// let server_socket = TcpListener::bind("127.0.0.1:-1").unwrap();
    /// ```
    #[error("bind socket")]
    Bind(#[source] std::io::Error),

    /// The error occurs when the socket is not accepted correctly
    ///
    /// Please, see [it](std::net::TcpListener::accept)
    ///
    /// **Do not confuse with [`UdtError::TimeoutExpired`]!**
    #[error("accept socket")]
    Accept(#[source] std::io::Error),

    /// This error occurs when the connection is wrong
    ///
    /// Please, see [it](std::net::TcpStream::connect)
    ///
    /// **Do not confuse with [`UdtError::TimeoutExpired`]!**
    #[error("connection to socket")]
    Connect(#[source] std::io::Error),

    /// This error occurs when the files are not working properly
    ///
    /// For example: `You do not have permission to write to the file`
    #[error("IO filesystem")]
    FileIO(#[source] std::io::Error),

    /// This error occurs if we cannot receive something from the socket
    ///
    /// Please, see [it](std::io::Read::read) and [it](std::net::TcpStream)
    #[error("receiving data")]
    ReceivingData(#[source] std::io::Error),

    /// Handshake error
    #[error("handshake")]
    Handshake(#[from] HandshakeError),

    /// Checksum is different
    #[error("file invalid")]
    FileInvalid,

    /// Each operation that is connected to the network has time limit
    ///
    /// This is the timeout
    #[error("timeout expired")]
    TimeoutExpired,

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
