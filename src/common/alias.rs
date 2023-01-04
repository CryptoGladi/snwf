use std::sync::{Arc, Mutex};

/// Enum for detect progress
///
/// **DON'T SHOW ERROR!**
#[derive(Debug)]
pub enum Progressing {
    Yield {
        done_files: u64,
        total_bytes: u64,
        done_bytes: u64,
    },
    Done,
}

pub type ProgressFn<'a> = Arc<Mutex<Box<dyn FnMut(Progressing) + 'a>>>;
