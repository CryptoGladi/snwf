//! Prelude for [`rsync`](crate::protocol::rsync)

use super::*;

pub(crate) use constant::*;
pub(crate) use error::assert_rsync;
pub use error::RSyncError;
pub use rsync_recipient::RSyncRecipient;
pub use rsync_sender::RSyncSender;
