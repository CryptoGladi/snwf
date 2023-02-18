//! Useful constant

use std::time::Duration;
use tokio_udt::UdtConfiguration;

mod from_udt_tokio {
    pub(crate) const DEFAULT_MSS: u32 = 1500;
    pub(crate) const DEFAULT_UDT_BUF_SIZE: u32 = 81920;
    pub(crate) const DEFAULT_UDP_BUF_SIZE: usize = 81920; // CHANGED!
}

/// Config for [`tokio-udt`](https://github.com/Distributed-EPFL/tokio-udt)
///
/// Solves a [bug](https://github.com/Distributed-EPFL/tokio-udt/issues/2) in MacOS
pub(crate) const UDT_CONFIGURATION: Option<UdtConfiguration> = Some(UdtConfiguration {
    mss: from_udt_tokio::DEFAULT_MSS,
    flight_flag_size: 256_000,
    snd_buf_size: from_udt_tokio::DEFAULT_UDT_BUF_SIZE,
    rcv_buf_size: from_udt_tokio::DEFAULT_UDT_BUF_SIZE * 2,
    udp_snd_buf_size: from_udt_tokio::DEFAULT_UDP_BUF_SIZE,
    udp_rcv_buf_size: from_udt_tokio::DEFAULT_UDP_BUF_SIZE,
    udp_reuse_port: false,
    linger_timeout: Some(Duration::from_secs(10)),
    reuse_mux: true,
    rendezvous: false,
    accept_queue_size: 1000,
});
