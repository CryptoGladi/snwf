use std::{net::{Ipv4Addr, ToSocketAddrs}, path::{PathBuf, Path}, fs::{read_to_string}};
use crate::{sender::{CoreSender, Sender}, stream::progressing_read};
use async_trait::async_trait;
use log::debug;
use thiserror::Error;
use tokio::io::{BufReader, AsyncReadExt};
use tokio::fs::File;
use tokio_udt::{UdtListener, UdtConnection};

#[derive(Debug, Error)]
pub enum UdtError {
    #[error("")]
    Bind(#[source] std::io::Error),
    
    #[error("I3")]
    Accept(#[source] std::io::Error),

    #[error("")]
    IO(#[source] std::io::Error),
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

    async fn udt_recv_file(&mut self, output: impl AsRef<Path> + Send) -> Result<(), UdtError>;
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
        let port = self.get_port();
        let target = self.get_target();

        let mut connection = UdtConnection::connect((target, port), None).await.unwrap();
        let mut reader = BufReader::new(file);

        if let Err(error) = tokio::io::copy(&mut reader, &mut connection).await {
            return Err(UdtError::IO(error));
        }

        Ok(())
    }

    async fn udt_recv_file(&mut self, output: impl AsRef<Path> + Send) -> Result<(), UdtError> {
        let port = self.get_port();
        let udt_listener = match UdtListener::bind((Ipv4Addr::UNSPECIFIED, port).into(), None).await {
            Ok(result) => result, // TODO Сделать из этого макрос и как отдельную библиотеку
            Err(error) => return Err(UdtError::Bind(error))
        };
    
        println!("Waiting for connections...");
    
        loop {
            let (addr, mut connection) = udt_listener.accept().await.unwrap();
            println!("Accepted connection from {}", addr);
            let mut buffer = Vec::with_capacity(1_000_000);
            tokio::task::spawn({
                async move {
                    loop {
                        match connection.read_buf(&mut buffer).await {
                            Ok(_size) => {}
                            Err(e) => {
                                eprintln!("Connnection with {} failed: {}", addr, e);
                                break;
                            }
                        }
                    }
                }
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn send_and_recv() {
        crate::init_logger_for_test();

        let (_temp_dir, path_to_file) = file_hashing::fs::extra::generate_random_file(22222);
        let mut sender = Sender::new("0.0.0.0:1111".parse().unwrap(), 5425);
        let mut file = File::open(path_to_file).await.unwrap();

        sender.udt_send_file(&mut file).await.unwrap();
        //sender.udt_recv_file(output);
    }
}