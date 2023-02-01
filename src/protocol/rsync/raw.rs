use super::prelude::*;
use crate::{
    common::DEFAULT_BUFFER_SIZE_FOR_NETWORK,
    prelude::{ConfigRecipient, ConfigSender, ProtocolError::*},
};
use log::{debug, trace};
use std::path::Path;
use tokio::{
    fs::File,
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};
use tokio_udt::{UdtConnection, UdtListener};

pub(crate) async fn bind_all(
    config: &'_ ConfigRecipient<'_>,
) -> Result<(UdtListener, TcpListener), RSyncError> {
    debug!("run bind_all for rsync");

    let udt_listener = UdtListener::bind((config.addr, config.port_for_send_files).into(), None)
        .await
        .map_err(|e| RSyncError::Protocol(Bind(e)))?;
    debug!("done bind udt_listener");

    let tcp_listener = TcpListener::bind((config.addr, config.port_for_handshake))
        .await
        .map_err(|e| RSyncError::Protocol(Bind(e)))?;
    debug!("done bind tcp_listener");

    Ok((udt_listener, tcp_listener))
}

pub(crate) async fn connect_all(
    config: &'_ ConfigSender<'_>,
) -> Result<(UdtConnection, TcpStream), RSyncError> {
    let udt_connection = UdtConnection::connect((config.addr, config.port_for_send_files), None)
        .await
        .map_err(|e| RSyncError::Protocol(Connect(e)))?;

    let tcp_connection = TcpStream::connect((config.addr, config.port_for_handshake))
        .await
        .map_err(|e| RSyncError::Protocol(Connect(e)))?;

    Ok((udt_connection, tcp_connection))
}

pub(crate) async fn send_signature(
    path: impl AsRef<Path>,
    udt_connection: &UdtConnection,
) -> Result<(), RSyncError> {
    let mut storage = vec![0u8; DEFAULT_BUFFER_SIZE_FOR_NETWORK];
    let mut buf = vec![0u8; DEFAULT_BUFFER_SIZE_FOR_NETWORK];
    let mut file = File::open(path)
        .await
        .map_err(|e| RSyncError::Protocol(IO(e)))?;

    loop {
        let len = file
            .read_buf(&mut buf)
            .await
            .map_err(|e| RSyncError::Protocol(IO(e)))?;

        if len == 0 {
            break;
        }

        fast_rsync::Signature::calculate(&buf[..], &mut storage, SIGNATURE_OPTIONS);
        udt_connection
            .send(&storage)
            .await
            .map_err(|e| RSyncError::Protocol(IO(e)))?;

        trace!("done send one block file. len: {len}");
    }

    udt_connection
        .send(STOP_WORD)
        .await
        .map_err(|e| RSyncError::Protocol(IO(e)))?;
    Ok(())
}

pub(crate) async fn get_big_message(udt_connection: &UdtConnection) -> Result<Vec<u8>, RSyncError> {
    let mut buf = vec![0u8; DEFAULT_BUFFER_SIZE_FOR_NETWORK];
    let mut result = vec![0u8; DEFAULT_BUFFER_SIZE_FOR_NETWORK];

    loop {
        let len = udt_connection.recv(&mut buf).await.map_err(|e| RSyncError::Protocol(IO(e)))?;

        if len == 0 || buf[len] == STOP_WORD {
            break;
        }

        result.append(&mut buf[0..len]);
    }


    /// TODO WRONG
    Ok((result))
}

// TODO Add timeout