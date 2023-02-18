//! More detailed functions for [`udt`](crate::protocol::udt)

use super::prelude::UdtError;
use crate::{
    common::timeout,
    prelude::{ConfigRecipient, ConfigSender},
    protocol::{error::ProtocolError::*, udt::contains::UDT_CONFIGURATION},
};
use log::debug;
use tokio::net::{TcpListener, TcpStream};
use tokio_udt::{UdtConnection, UdtListener};

/// Make all connections for [`Sender`](crate::sender::Sender)
pub(crate) async fn all_connect_for_sender(
    config: &ConfigSender<'_>,
) -> Result<(UdtConnection, TcpStream), UdtError> {
    debug!("run all_connect_for_sender for udt. Config: {:?}", config);

    let udt_connection = timeout!(
        UdtConnection::connect((config.addr, config.port_for_send_files), UDT_CONFIGURATION),
        |_| UdtError::Protocol(TimeoutExpired),
        config.timeout
    )?
    .map_err(|e| UdtError::Protocol(Connect(e)))?;
    debug!("done socket udt connect");

    let socket_for_handshake = timeout!(
        TcpStream::connect((config.addr, config.port_for_handshake)),
        |_| UdtError::Protocol(TimeoutExpired),
        config.timeout
    )?
    .map_err(|e| UdtError::Protocol(Connect(e)))?;
    debug!("done socket handshake connect");

    Ok((udt_connection, socket_for_handshake))
}

/// Make bind connections for [`Recipient`](crate::recipient::Recipient)
pub(crate) async fn all_bind_for_recipient(
    config: &ConfigRecipient<'_>,
) -> Result<(UdtListener, TcpListener), UdtError> {
    debug!("run all_bind_for_recipient for udt. Config: {:?}", config);

    let udt_listener = UdtListener::bind(
        (config.addr, config.port_for_send_files).into(),
        UDT_CONFIGURATION,
    )
    .await
    .map_err(|e| UdtError::Protocol(Bind(e)))?;
    debug!("done socket udt bind");

    let tcp_handshake = TcpListener::bind((config.addr, config.port_for_handshake))
        .await
        .map_err(|e| UdtError::Protocol(Bind(e)))?;
    debug!("done socket handshake bind");

    Ok((udt_listener, tcp_handshake))
}
