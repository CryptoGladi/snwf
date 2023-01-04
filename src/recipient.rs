//! Module for [`Recipient`]

use crate::common::{generate_config, generate_new_for_config, Progressing};
use std::sync::{Arc, Mutex};

generate_config!(ConfigRecipient, Recipient);

/// Core trait for [`Recipient`]
pub trait CoreRecipient<'a> {
    /// Get [`ConfigRecipient`]
    fn get_config(&self) -> ConfigRecipient<'a>;

    /// Set ['ProgressFnT']
    fn set_progress_fn(&mut self, progress_fn: Option<impl FnMut(Progressing) + 'a>);
}

/// Main implementation for [`CoreRecipient`]
///
/// ## Warning
///
/// Only stores connection information. No protocol implementation!
pub struct Recipient<'a> {
    config: ConfigRecipient<'a>,
}

impl Recipient<'static> {
    generate_new_for_config!(ConfigRecipient);
}

impl<'a> CoreRecipient<'a> for Recipient<'a> {
    /// Get [`ConfigRecipient`]
    fn get_config(&self) -> ConfigRecipient<'a> {
        self.config.clone()
    }

    /// Set ['ProgressFnT']
    fn set_progress_fn(&mut self, progress_fn: Option<impl FnMut(Progressing) + 'a>) {
        self.config.progress_fn = if let Some(progress_fn) = progress_fn {
            Some(Arc::new(Mutex::new(Box::new(progress_fn))))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::udt::*;
    use std::path::Path;

    #[tokio::test]
    async fn test_progress_fn_set() {
        let mut recipient = Recipient::new("::1".parse().unwrap(), 5344, 4236);
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
