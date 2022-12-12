use std::net::{IpAddr, ToSocketAddrs};
use derive_new::new;

pub type DoneBytes = usize;

pub trait CoreSender {
    fn get_target(&self) -> IpAddr;

    fn get_port(&self) -> u16;
}

#[derive(new)]
pub struct Sender
{
    target: IpAddr,
    port: u16,
}

impl CoreSender for Sender {
    fn get_target(&self) -> IpAddr {
        self.target
    }

    fn get_port(&self) -> u16 {
        self.port
    }
}
