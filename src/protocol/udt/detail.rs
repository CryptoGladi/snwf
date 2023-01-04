use super::UdtError;
use crate::{common::timeout, prelude::ConfigSender};
use log::debug;
use tokio::net::TcpStream;
use tokio_udt::UdtConnection;

pub(crate) async fn connect_for_sender(
    config: &ConfigSender<'_>,
) -> Result<(UdtConnection, TcpStream), UdtError> {
    debug!("run join_connect for udt");

    let udt_connection = timeout!(
        UdtConnection::connect((config.addr, config.port_for_send_files), None),
        |_| UdtError::TimeoutExpired,
        config.timeout
    )?
    .map_err(UdtError::Connect)?;
    debug!("done socket udt connect");

    let socket_for_handshake = timeout!(
        TcpStream::connect((config.addr, config.port_for_handshake)),
        |_| UdtError::TimeoutExpired,
        config.timeout
    )?
    .map_err(UdtError::Connect)?;
    debug!("done socket handshake connect");

    Ok((udt_connection, socket_for_handshake))
}
