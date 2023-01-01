use crate::common::{generate_config, generate_new_for_config};

generate_config!(ConfigSender);

pub trait CoreSender {
    fn get_config(&self) -> ConfigSender;
}

#[derive(Debug)]
pub struct Sender {
    config: ConfigSender,
}

impl Sender {
    generate_new_for_config!(ConfigSender);
}

impl CoreSender for Sender {
    fn get_config(&self) -> ConfigSender {
        self.config.clone()
    }
}
