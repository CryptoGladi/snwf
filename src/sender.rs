use derive_new::new;
use std::{
    hash::Hasher,
    net::IpAddr,
    sync::{Arc, Mutex},
};
use digest::DynDigest;

pub type DoneBytes = usize;

pub trait CoreSender {
    fn get_target(&self) -> IpAddr;

    fn get_port(&self) -> u16;

    fn get_hasher(&self) -> Arc<Mutex<dyn DynDigest + Send>>;
}

#[derive(new)]
pub struct Sender {
    target: IpAddr,
    port: u16,
    hasher: Arc<Mutex<dyn DynDigest + Send>>,
}

impl CoreSender for Sender {
    fn get_target(&self) -> IpAddr {
        self.target
    }

    fn get_port(&self) -> u16 {
        self.port
    }

    fn get_hasher(&self) -> Arc<Mutex<dyn DynDigest + Send>> {
        self.hasher.clone()
    }
}
