use crate::{
    sender::{CoreSender, Sender},
    stream::progressing_read,
};
use async_trait::async_trait;
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
use tokio::{
    fs::{File, OpenOptions},
    time::timeout,
};
use tokio_udt::{UdtConnection, UdtListener};

const TIMEOUT: Duration = Duration::from_millis(1000);

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
    ConnectionLost,
}

#[derive(Debug)]
pub struct UdtConfig {

}

#[async_trait]
pub trait UdtSender: CoreSender {
    async fn udt_send_file(&mut self, file: &mut File) -> Result<(), UdtError>;

    async fn udt_recv_file<P: AsRef<Path> + Send + Copy>(
        &mut self,
        output: P,
    ) -> Result<(), UdtError>;
}

#[async_trait]
impl UdtSender for Sender {
    async fn udt_send_file(&mut self, file: &mut File) -> Result<(), UdtError> {
        debug!("Running udt_send_file...");
        let info_for_connect = (self.get_target(), self.get_port());

        let mut connection = UdtConnection::connect(info_for_connect, None)
            .await
            .map_err(UdtError::Connect)?;

        let mut reader = BufReader::new(file);
        debug!("Done connect in udt");

        // TODO https://stackoverflow.com/questions/71632833/how-to-continuously-watch-and-read-file-asynchronously-in-rust-using-tokio
        tokio::io::copy(&mut reader, &mut connection)
            .await
            .map_err(UdtError::FileIO)?;

        debug!("Done send file in udt");
        Ok(())
    }

    async fn udt_recv_file<P: AsRef<Path> + Send + Copy>(
        &mut self,
        output: P,
    ) -> Result<(), UdtError> {
        let udt_listener = UdtListener::bind((Ipv6Addr::UNSPECIFIED, self.get_port()).into(), None).await.map_err(UdtError::Bind)?;

        let mut file_for_write = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(output)
            .await
            .map_err(UdtError::FileIO)?;

        let (addr, mut connection) = udt_listener.accept().await.map_err(UdtError::Accept)?;
        debug!("Accepted connection from {}", addr);

        if let Ok(copying) = timeout(TIMEOUT, tokio::io::copy(&mut connection, &mut file_for_write)).await {
            copying.map_err(UdtError::FileIO)?;
            return Ok(());
        };
        Ok(()) // TODO Подклёчение разовралось или всё файлы успешно доставлены?

        /*
        loop {
            match timeout(TIMEOUT, connection.read_buf(&mut buffer)).await {
                Ok(len) => {
                    let Ok(len) = len else {
                        return Err(UdtError::ConnectionLost);
                    };

                    file_for_write.write_all(&buffer[0..len]).await.unwrap();
                }
                Err(_) => {
                    debug!("Timeout end = end send file");
                    file_for_write.flush().await.unwrap();
                    return Ok(());
                }
            }
        } */
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn send_and_recv() {
        crate::init_logger_for_test();

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

        println!("DONE");
    }
}
