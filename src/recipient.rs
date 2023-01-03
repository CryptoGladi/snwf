//! Module for [`Recipient`]

use crate::common::{
    generate_config, generate_new_for_config, generate_set_progress_fn_for_config,
};

generate_config!(ConfigRecipient, Recipient);

/// Core trait for [`Recipient`]
pub trait CoreRecipient {
    fn get_config(&self) -> ConfigRecipient;
}

/// Main implementation for [`CoreRecipient`]
///
/// ## Warning
///
/// Only stores connection information. No protocol implementation!
#[derive(Debug)]
pub struct Recipient {
    config: ConfigRecipient,
}

impl Recipient {
    generate_new_for_config!(ConfigRecipient);

    generate_set_progress_fn_for_config!();
}

impl CoreRecipient for Recipient {
    /// Get [`ConfigRecipient`]
    fn get_config(&self) -> ConfigRecipient {
        self.config.clone()
    }
}
