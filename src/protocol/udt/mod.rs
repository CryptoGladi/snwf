use crate::common::{get_hasher, TIMEOUT};
use crate::protocol::handshake::*;
use crate::recipient::{CoreRecipient, Recipient};
use crate::sender::{CoreSender, Sender};
use async_trait::async_trait;
use blake2::{Blake2b512, Digest};
use derive_new::new;
use log::debug;
use std::{
    fs::read_to_string,
    net::Ipv6Addr,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::{
    fs::{File, OpenOptions},
    time::timeout,
};
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

    #[error("handshake")]
    Handshake(#[from] HandshakeError),

    #[error("file invalid. Check network")]
    FileInvalid,
}

#[async_trait]
pub trait UdtSender: CoreSender {
    async fn udt_send_file<P, A>(&mut self, path: P, tcp_address: A) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync,
        A: ToSocketAddrs + Send;
}

#[async_trait]
impl UdtSender for Sender {
    async fn udt_send_file<P, A>(&mut self, path: P, tcp_address: A) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync,
        A: ToSocketAddrs + Send,
    {
        let info_for_connect = (self.get_target(), self.get_port());

        let mut udt = UdtConnection::connect(info_for_connect, None)
            .await
            .map_err(UdtError::Connect)?;
        let mut socket_for_handshake = TcpStream::connect(tcp_address)
            .await
            .map_err(UdtError::Connect)?;

        raw::send_file(&mut udt, path, &mut socket_for_handshake).await?;

        Ok(())
    }
}

#[async_trait]
pub trait UdtRecipient: CoreRecipient {
    async fn udt_recv_file<P, A>(&mut self, output: P, bind_tcp_addr: A) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync,
        A: ToSocketAddrs + Send;
}

#[async_trait]
impl UdtRecipient for Recipient {
    async fn udt_recv_file<P, A>(&mut self, output: P, bind_tcp_addr: A) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Send + Copy + Sync,
        A: ToSocketAddrs + Send,
    {
        let udt_listener = UdtListener::bind((Ipv6Addr::UNSPECIFIED, self.get_port()).into(), None)
            .await
            .map_err(UdtError::Bind)?;
        let mut tcp_handshake = TcpListener::bind(bind_tcp_addr)
            .await
            .map_err(UdtError::Bind)?;

        let (addr, mut connection) = udt_listener.accept().await.map_err(UdtError::Accept)?;
        debug!("Accepted connection from {}", addr);

        raw::recv_file(&mut connection, &mut tcp_handshake, output).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use super::*;

    pub(crate) mod detail {
        use super::*;
        use std::net::IpAddr;

        pub(crate) async fn send(
            target: IpAddr,
            port: u16,
            path: &Path,
            tcp_address: impl ToSocketAddrs + Send,
        ) -> Result<(), UdtError> {
            let mut sender = Sender::new(target, port);
            sender.udt_send_file(path, tcp_address).await?;

            Ok(())
        }

        pub(crate) async fn recv(
            target: IpAddr,
            port: u16,
            output: &Path,
            bind_tcp_addr: impl ToSocketAddrs + Send,
        ) -> Result<(), UdtError> {
            let mut recipient = Recipient::new(target, port);
            recipient.udt_recv_file(output, bind_tcp_addr).await?;

            Ok(())
        }
    }

    #[tokio::test]
    async fn send_and_recv() {
        crate::init_logger_for_test();
        // TODO Сделать везде timeout

        const TARGET: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
        const PORT: u16 = 5824;
        const BIND_TCP_ADDRESS: &'static str = "";
        const TCP_ADDRESS: &'static str = "";

        let (temp_dir, path_input) = file_hashing::fs::extra::generate_random_file(4352);
        let path_output = temp_dir.join("tess_file.txt");

        let (send, recv) = tokio::join!(
            detail::send(TARGET, PORT, path_input.path(), TCP_ADDRESS),
            detail::recv(TARGET, PORT, path_output.as_path(), BIND_TCP_ADDRESS)
        );
        send.unwrap();
        recv.unwrap();
    }
}
