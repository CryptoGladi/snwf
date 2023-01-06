//! Module for [`Sender`]

use crate::common::{generate_config, generate_new_for_config, Progressing};
use std::sync::{Arc, Mutex};

generate_config!(ConfigSender, Sender);

/// Core trait for [`Sender`]
pub trait CoreSender<'a> {
    /// Get [`ConfigSender`]
    fn get_config(&self) -> ConfigSender<'a>;

    /// Set ['ProgressFnT']
    fn set_progress_fn(&mut self, progress_fn: Option<impl FnMut(Progressing) + 'a>);
}

/// Main implementation for [`CoreSender`]
///
/// ## Warning
///
/// Only stores connection information. No protocol implementation!
pub struct Sender<'a> {
    config: ConfigSender<'a>,
}

impl<'a> Sender<'a> {
    generate_new_for_config!(ConfigSender);
}

impl<'a> CoreSender<'a> for Sender<'a> {
    /// Get [`ConfigSender`]
    fn get_config(&self) -> ConfigSender<'a> {
        self.config.clone()
    }

    /// Set ['ProgressFnT']
    fn set_progress_fn(&mut self, progress_fn: Option<impl FnMut(Progressing) + 'a>) {
        self.config.progress_fn = progress_fn
            .map(|i| -> crate::common::alias::ProgressFn { Arc::new(Mutex::new(Box::new(i))) });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_fn_set() {
        let mut recipient = Sender::new("127.0.0.1".parse().unwrap(), 5344, 4236);
        let test_value = Arc::new(Mutex::new(43));

        {
            let test_value_clone = test_value.clone();
            recipient.set_progress_fn(Some(move |_progressing| {
                *test_value_clone.lock().unwrap() += 1;
            }));
        }

        recipient.config.progress_fn.unwrap().lock().unwrap()(Progressing::Done);
        assert_eq!(*test_value.lock().unwrap(), 44);
    }
}
