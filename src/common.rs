use blake2::{Blake2b512, Digest};
use std::time::Duration;

pub(crate) const TIMEOUT: Duration = Duration::from_millis(1000);

macro_rules! timeout {
    ($x:expr, $e:expr) => {
        tokio::time::timeout(crate::common::TIMEOUT, $x)
            .await
            .map_err($e)
    };
}

pub(crate) use timeout;

pub(crate) fn get_hasher() -> Blake2b512 {
    Blake2b512::new()
}

macro_rules! generate_config {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub addr: std::net::IpAddr,
            pub port_for_send_files: u16,
            pub port_for_handshake: u16,
            pub timeout: std::time::Duration,
        }
    };
}

pub(crate) use generate_config;

macro_rules! generate_new_for_config {
    ($name_config:ident) => {
        pub fn new(
            addr: std::net::IpAddr,
            port_for_send_files: u16,
            port_for_handshake: u16,
        ) -> Self {
            Self {
                config: $name_config {
                    addr,
                    port_for_send_files,
                    port_for_handshake,
                    timeout: crate::common::TIMEOUT,
                },
            }
        }
    };
}

pub(crate) use generate_new_for_config;
