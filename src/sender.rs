//! Module for [`Sender`]

use crate::common::{
    generate_config, generate_new_for_config, generate_set_progress_fn_for_config,
};

generate_config!(ConfigSender, Sender);

/// Core trait for [`Sender`]
pub trait CoreSender {
    /// Get [`ConfigSender`]
    fn get_config(&self) -> ConfigSender;
}

/// Main implementation for [`CoreSender`]
///
/// ## Warning
///
/// Only stores connection information. No protocol implementation!
#[derive(Debug)]
pub struct Sender {
    config: ConfigSender,
}

impl Sender {
    generate_new_for_config!(ConfigSender);

    generate_set_progress_fn_for_config!();
}

impl CoreSender for Sender {
    /// Get [`ConfigSender`]
    fn get_config(&self) -> ConfigSender {
        self.config.clone()
    }
}
