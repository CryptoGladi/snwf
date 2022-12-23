use blake2::Blake2s256;
use derive_new::new;
use std::{
    hash::Hasher,
    net::IpAddr,
    sync::{Arc, Mutex},
};

pub trait CoreSender {
    fn get_target(&self) -> IpAddr;

    fn get_port(&self) -> u16;
}

#[derive(Debug, new)]
pub struct Sender {
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
