use derive_new::new;
use std::net::{IpAddr, ToSocketAddrs};

pub type DoneBytes = usize;

pub trait CoreRecipient {
    fn get_target(&self) -> IpAddr;

    fn get_port(&self) -> u16;
}

#[derive(new)]
pub struct Recipient {
    target: IpAddr,
    port: u16,
}

impl CoreRecipient for Recipient {
    fn get_target(&self) -> IpAddr {
        self.target
    }

    fn get_port(&self) -> u16 {
        self.port
    }
}
