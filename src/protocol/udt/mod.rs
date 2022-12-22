use crate::common::{get_hasher, TIMEOUT};
use crate::protocol::handshake::*;
use crate::sender::{CoreSender, Sender};
use async_trait::async_trait;
use blake2::{Blake2b512, Digest};
use derive_new::new;
use log::debug;
use std::{
    fs::read_to_string,
    net::{Ipv6Addr, ToSocketAddrs},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::{
    fs::{File, OpenOptions},
    time::timeout,
};
use tokio_udt::{UdtConnection, UdtListener};

mod detail;

#[derive(Debug, Error)]
pub enum UdtError {
    #[error("")]
    Bind(#[source] std::io::Error),

    #[error("I3")]
    Accept(#[source] std::io::Error),

    #[error("")]
    Connect(#[source] std::io::Error),

    #[error("")]
    FileIO(#[source] std::io::Error),

    #[error("")]
    Handshake(#[from] HandshakeError),

    #[error("")]
    FileInvalid,
}

#[async_trait]
pub trait UdtSender: CoreSender {
    async fn udt_send_file<P>(&mut self, path: P) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Sync + Copy + Send;

    async fn udt_recv_file<P: AsRef<Path> + Send + Copy>(
        &mut self,
        output: P,
    ) -> Result<(), UdtError>;
}

#[async_trait]
impl UdtSender for Sender {
    async fn udt_send_file<P>(&mut self, path: P) -> Result<(), UdtError>
    where
        P: AsRef<Path> + Sync + Copy + Send,
    {
        let file = File::open(path).await.map_err(UdtError::FileIO)?;
        let info_for_connect = (self.get_target(), self.get_port());

        let mut udt = UdtConnection::connect(info_for_connect, None)
            .await
            .map_err(UdtError::Connect)?;
        let mut socket_for_handshake = TcpStream::connect("127.0.0.1:4725")
            .await
            .map_err(UdtError::Accept)?;
        let mut reader = BufReader::new(file);

        detail::raw_send_file(&mut udt, path, &mut socket_for_handshake);

        todo!()
    }

    async fn udt_recv_file<P: AsRef<Path> + Send + Copy>(
        &mut self,
        output: P,
    ) -> Result<(), UdtError> {
        let udt_listener = UdtListener::bind((Ipv6Addr::UNSPECIFIED, self.get_port()).into(), None)
            .await
            .map_err(UdtError::Bind)?;

        let mut file_for_write = OpenOptions::new()
            .create(true)
            .write(true)
            .open(output)
            .await
            .map_err(UdtError::FileIO)?;

        let (addr, mut connection) = udt_listener.accept().await.map_err(UdtError::Accept)?;
        debug!("Accepted connection from {}", addr);

        if let Ok(copying) = timeout(
            TIMEOUT,
            tokio::io::copy(&mut connection, &mut file_for_write),
        )
        .await
        {
            copying.map_err(UdtError::FileIO)?;
            return Ok(());
        };
        Ok(()) // TODO Подклёчение разовралось или всё файлы успешно доставлены?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn send_and_recv() {
        crate::init_logger_for_test();

        /*
        tokio::spawn(async {
            let mut file = File::open("/home/gladi/test_for_send.txt").await.unwrap();
            let mut sender = Sender::new("127.0.0.1".parse().unwrap(), 5425);
            sender.udt_send_file(&mut file).await.unwrap();
        });

        tokio::spawn(async {
            let mut sender = Sender::new("127.0.0.1".parse().unwrap(), 5425);
            sender
                .udt_recv_file(Path::new("/home/gladi/test_file.txt"))
                .await
                .unwrap();
        })
        .await
        .unwrap();
        */

        println!("DONE");
    }
}
