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

pub trait ProgressFnT: FnMut(Progressing) {}

impl<F> ProgressFnT for F where F: FnMut(Progressing) {}

impl std::fmt::Debug for dyn ProgressFnT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProgressFn")
    }
}

pub type ProgressFn = Arc<Mutex<Box<dyn ProgressFnT>>>;
