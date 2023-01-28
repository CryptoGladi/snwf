use crate::prelude::{ConfigRecipient, ProtocolError};
use tokio::net::TcpListener;
use tokio_udt::UdtListener;

use super::RSyncError;

pub(crate) async fn bind_all(
    config: &'_ ConfigRecipient<'_>,
) -> Result<(UdtListener, TcpListener), RSyncError> {
    let udt_listener = UdtListener::bind((config.addr, config.port_for_handshake).into(), None)
        .await
        .map_err(|e| RSyncError::Protocol(ProtocolError::Bind(e)))?;
    let tcp_listener = TcpListener::bind((config.addr, config.port_for_handshake))
        .await
        .map_err(|e| RSyncError::Protocol(ProtocolError::Bind(e)))?;

    Ok((udt_listener, tcp_listener))
}
