//! Module for [`Recipient`]

use crate::common::{
    generate_config, generate_new_for_config, ProgressFnT
};

generate_config!(ConfigRecipient, Recipient);

/// Core trait for [`Recipient`]
pub trait CoreRecipient<'a> {
    /// Get [`ConfigRecipient`]
    fn get_config(&'a self) -> ConfigRecipient<'a>;

    /// Set ['ProgressFnT']
    fn set_progress_fn(&mut self, progress_fn: Box<dyn ProgressFnT + 'a>);
}

/// Main implementation for [`CoreRecipient`]
///
/// ## Warning
///
/// Only stores connection information. No protocol implementation!
pub struct Recipient<'a> {
    config: ConfigRecipient<'a>,
}

impl Recipient<'_> {
    generate_new_for_config!(ConfigRecipient);
}

impl<'a> CoreRecipient<'a> for Recipient<'a> {
    /// Get [`ConfigRecipient`]
    fn get_config(&'a self) -> ConfigRecipient<'a> {
        self.config.clone()
    }

    /// Set ['ProgressFnT']
    fn set_progress_fn(&mut self, progress_fn: Box<dyn ProgressFnT + 'a>) {
         self.config.progress_fn = Some(std::sync::Arc::new(std::sync::Mutex::new(progress_fn)));
    }
}
