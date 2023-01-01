use crate::common::timeout;
use crate::protocol::handshake::*;
use crate::recipient::{CoreRecipient, Recipient};
use crate::sender::{CoreSender, Sender};
use async_trait::async_trait;
use log::debug;
use std::path::Path;
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio_udt::{UdtConnection, UdtListener};

mod raw;

#[derive(Debug, Error)]
pub enum UdtError {
    #[error("bind socket")]
    Bind(#[source] std::io::Error),

    #[error("accept sender")]
    Accept(#[source] std::io::Error),

    #[error("connection to recipient")]
    Connect(#[source] std::io::Error),

    #[error("IO filesystem")]
    FileIO(#[source] std::io::Error),

    #[error("IO network")]
    NetworkIO(#[source] std::io::Error),

    #[error("handshake")]
    Handshake(#[from] HandshakeError),

    #[error("file invalid. Check network")]
    FileInvalid,

    #[error("timeout expired")]
    TimeoutExpired,
}

#[async_trait]
pub trait UdtSender: CoreSender {
    async fn udt_send_file<P>(&mut self, path: P) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync;
}

#[async_trait]
impl UdtSender for Sender {
    async fn udt_send_file<P>(&mut self, path: P) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync,
    {
        let config = self.get_config();
        debug!("running udt_send_file; config: {:?}", config);

        let mut udt = timeout!(
            UdtConnection::connect((config.addr, config.port_for_send_files), None),
            |_| UdtError::TimeoutExpired
        )?
        .map_err(UdtError::Connect)?;
        debug!("done socket udt connect");

        let mut socket_for_handshake = timeout!(
            TcpStream::connect((config.addr, config.port_for_handshake)),
            |_| UdtError::TimeoutExpired
        )?
        .map_err(UdtError::Connect)?;
        debug!("done socket handshake connect");

        raw::send_file(&mut udt, path, &mut socket_for_handshake).await?;

        Ok(())
    }
}

#[async_trait]
pub trait UdtRecipient: CoreRecipient {
    async fn udt_recv_file<P>(&mut self, output: P) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync;
}

#[async_trait]
impl UdtRecipient for Recipient {
    async fn udt_recv_file<P>(&mut self, output: P) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync,
    {
        let config = self.get_config();
        debug!("running udt_recv_file; config: {:?}", config);

        let udt_listener =
            UdtListener::bind((config.addr, config.port_for_send_files).into(), None)
                .await
                .map_err(UdtError::Bind)?;
        debug!("done socket udt bind");

        let mut tcp_handshake = TcpListener::bind((config.addr, config.port_for_handshake))
            .await
            .map_err(UdtError::Bind)?;
        debug!("done socket handshake bind");

        let (addr, mut connection) = timeout!(udt_listener.accept(), |_| UdtError::TimeoutExpired)?
            .map_err(UdtError::Accept)?;
        debug!("accepted connection from {}", addr);

        raw::recv_file(&mut connection, &mut tcp_handshake, output).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::common::get_hasher;

    #[tokio::test]
    async fn send_and_recv_udt() {
        crate::init_logger_for_test();

        let (temp_dir, path_input) = file_hashing::fs::extra::generate_random_file(4352);
        let path_output = temp_dir.join("tess_file.txt");

        let mut sender = Sender::new("127.0.0.1".parse().unwrap(), 4324, 6343);
        let mut recipient = Recipient::new("::0".parse().unwrap(), 4324, 6343);

        let (recv, send) = tokio::join!(
            recipient.udt_recv_file(path_output.as_path()),
            sender.udt_send_file(path_input.path())
        );

        send.unwrap();
        recv.unwrap();

        let hash_input = file_hashing::get_hash_file(path_input, &mut get_hasher()).unwrap();
        let hash_output = file_hashing::get_hash_file(path_output, &mut get_hasher()).unwrap();

        assert_eq!(hash_input, hash_output)
    }
}
