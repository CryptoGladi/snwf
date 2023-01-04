//! Module for [`Recipient`]

use std::sync::{Arc, Mutex};
use crate::common::{generate_config, generate_new_for_config, Progressing};

generate_config!(ConfigRecipient, Recipient);

/// Core trait for [`Recipient`]
pub trait CoreRecipient<'a> {
    /// Get [`ConfigRecipient`]
    fn get_config(&self) -> ConfigRecipient<'a>;

    /// Set ['ProgressFnT']
    fn set_progress_fn(&mut self, progress_fn: &'a Box<dyn FnMut(Progressing) + 'a>);
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
    fn set_progress_fn(&mut self, progress_fn: &'a Box<dyn FnMut(Progressing) + 'a>) {
        self.config.progress_fn = Some(&Arc::new(Mutex::new(progress_fn)));
    }
}

#[cfg(test)]
mod tests {
    use super::{Recipient, CoreRecipient};

    #[test]
    fn test_progress_fn_set() {
        let mut recipient = Recipient::new("127.0.0.1".parse().unwrap(), 5344, 4236);

        let mut lol = 43;
        recipient.set_progress_fn(Box::new(|_| {
            lol += 1;
        }));

        let ttt = recipient.get_config();


    }
}