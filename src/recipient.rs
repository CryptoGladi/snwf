use crate::common::{generate_config, generate_new_for_config};

generate_config!(ConfigRecipient);

pub trait CoreRecipient {
    fn get_config(&self) -> ConfigRecipient;
}

#[derive(Debug)]
pub struct Recipient {
    config: ConfigRecipient,
}

impl Recipient {
    generate_new_for_config!(ConfigRecipient);
}

impl CoreRecipient for Recipient {
    fn get_config(&self) -> ConfigRecipient {
        self.config.clone()
    }
}
