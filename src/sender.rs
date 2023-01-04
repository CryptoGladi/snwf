//! Module for [`Sender`]

use crate::common::{
    generate_config, generate_new_for_config, ProgressFnT
};

generate_config!(ConfigSender, Sender);

/// Core trait for [`Sender`]
pub trait CoreSender<'a> {
    /// Get [`ConfigSender`]
    fn get_config(&'a self) -> ConfigSender<'a>;

    /// Set ['ProgressFnT']
    fn set_progress_fn(&mut self, progress_fn: Box<dyn ProgressFnT + 'a>);
}

/// Main implementation for [`CoreSender`]
///
/// ## Warning
///
/// Only stores connection information. No protocol implementation!
pub struct Sender<'a> {
    config: ConfigSender<'a>,
}

impl Sender<'_> {
    generate_new_for_config!(ConfigSender);
}

impl<'a> CoreSender<'a> for Sender<'a> {
    /// Get [`ConfigSender`]
    fn get_config(&'a self) -> ConfigSender<'a> {
        self.config.clone()
    }

    /// Set ['ProgressFnT']
    fn set_progress_fn(&mut self, progress_fn: Box<dyn ProgressFnT + 'a>) {
        self.config.progress_fn = Some(std::sync::Arc::new(std::sync::Mutex::new(progress_fn)));
    }
}
