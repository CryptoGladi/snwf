use crate::{
    sender::{CoreSender, Sender},
    stream::progressing_read,
};
use async_trait::async_trait;
use log::debug;
use std::{
    fs::read_to_string,
    net::{Ipv6Addr, ToSocketAddrs},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::{
    fs::{File, OpenOptions},
    time::timeout,
};
use tokio_udt::{UdtConnection, UdtListener};

#[derive(Debug, Error)]
pub enum UdtError {
    #[error("")]
    Bind(#[source] std::io::Error),

    #[error("I3")]
    Accept(#[source] std::io::Error),

    #[error("")]
    Connect(#[source] std::io::Error),

    #[error("")]
    A,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NumberOfIterations {
    Value(u32),
    Infinity,
}

#[derive(Debug)]
pub struct UdtConfig {
    number_of_iterations: NumberOfIterations,
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
    /* ЭТО ДЛЯ ПОЛУЧЕНИЕ ФАЙЛОВ
    async fn udt_send_files(&mut self, config: UdtConfig) -> Result<(), UdtError> {
        debug!("starting udt send files...");

        let port = self.get_port();
        let udt_listener = match UdtListener::bind((Ipv4Addr::UNSPECIFIED, port).into(), None).await {
            Ok(result) => result, // TODO Сделать из этого макрос и как отдельную библиотеку
            Err(error) => return Err(UdtError::Bind(error))
        };
        debug!("done udt listenet bind");

        let mut value_iterations = match config.number_of_iterations {
            NumberOfIterations::Value(value) => value,
            _ => 0, // is infinity
        };

        loop {
            if value_iterations == 0 && config.number_of_iterations != NumberOfIterations::Infinity {
                debug!("done send files in udt!");
                return Ok(());
            }

            let (socket_addr, udt_client) = match udt_listener.accept().await {
                Ok(result) => result,
                Err(error) => return Err(UdtError::Accept(error))
            };
            debug!("new udt recipient: {}", socket_addr);

            udt_client.send(msg)
            //progressing_read(reader, |len, bytes| {

            //});

            value_iterations -= 1;
        }
    }
    */

    async fn udt_send_file(&mut self, file: &mut File) -> Result<(), UdtError> {
        debug!("running udt_send_file...");
        let port = self.get_port();
        let target = self.get_target();
        let information_for_connect = (self.get_target(), self.get_port());

        //let mut connection = UdtConnection::connect((target, port), None).await.map_err(|e| Err(UdtError::Connect(e)))?;
    
        let mut connection = UdtConnection::connect(information_for_connect, None).await.unwrap_or_else(|e| Err(UdtError::Connect(e)));

        //f.read_to_string(&mut s).map_err(|e| MyCustomError:FileReadError(e))?;

        let mut reader = BufReader::new(file);
        debug!("done udt connect");

        if let Err(error) = tokio::io::copy(&mut reader, &mut connection).await {
            return Err(UdtError::IO(error));
        }

        debug!("done work");
        Ok(())
    }

    async fn udt_recv_file<P: AsRef<Path> + Send + Copy>(
        &mut self,
        output: P,
    ) -> Result<(), UdtError> {
        let port = self.get_port();
        let udt_listener = match UdtListener::bind((Ipv6Addr::UNSPECIFIED, port).into(), None).await
        {
            Ok(result) => result, // TODO Сделать из этого макрос и как отдельную библиотеку
            Err(error) => return Err(UdtError::Bind(error)),
        };

        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(output)
            .await
            .unwrap();

        let (addr, mut connection) = udt_listener.accept().await.unwrap();
        println!("Accepted connection from {}", addr);
        let mut buffer = Vec::with_capacity(4000);

        // TODO https://stackoverflow.com/questions/71632833/how-to-continuously-watch-and-read-file-asynchronously-in-rust-using-tokio
        loop {
            match timeout(Duration::from_millis(100), connection.read_buf(&mut buffer)).await {
                Ok(len) => {
                    let Ok(len) = len else {
                        return Err(UdtError::A);
                    };

                    file.write_all(&buffer[0..len]).await.unwrap();
                },
                Err(_) => {
                    file.flush().await.unwrap();
                    return Ok(());
                }
            }
        }
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
