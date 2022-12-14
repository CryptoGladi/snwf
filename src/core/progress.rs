use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

/// Callback to check the progress of the operation
///
/// # Example
///
/// ```
/// # use snwf::prelude::*;
/// #
/// let mut sender = Sender::new("127.0.0.1".parse().unwrap(), 4324, 6343);
/// sender.set_progress_fn(Some(|_progressing| { // add move for variable
///     // Useful user code
/// }));
/// ```
///
/// P.S. **DON'T SHOW ERROR!**
#[derive(Debug)]
pub enum Progressing {
    /// Progress Information
    Yield {
        /// How many files have already been sent or received?
        done_files: u64,

        /// How many total bytes to receive or send for a single file
        total_bytes: u64,

        /// Bytes received from a single file.
        ///
        /// That is, the file has not yet been completely sent or transferred
        done_bytes: u64,

        /// The path to the file that is involved in the work
        path_to_file: PathBuf,
    },

    /// Operation is done!
    ///
    /// P.S. **DON'T SHOW ERROR!**
    Done,
}

/// Alias for simple use FnMut([`Progressing`]) for struct
pub type ProgressFn<'a> = Arc<Mutex<Box<dyn FnMut(Progressing) + 'a>>>;
