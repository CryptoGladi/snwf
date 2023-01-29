//! Raw [udt](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) implementation

use super::prelude::UdtError;
use crate::{
    common::{
        get_hasher, timeout, DEFAULT_BUFFER_SIZE_FOR_FILE as FBUFFER_SIZE,
        DEFAULT_BUFFER_SIZE_FOR_NETWORK as NBUFFER_SIZE,
    },
    core::*,
    prelude::{ConfigRecipient, ConfigSender},
    protocol::{
        error::ProtocolError,
        handshake::{recv_handshake_from_address, send_handshake_from_file, Handshake},
    },
};
use log::debug;
use std::path::Path;
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{TcpListener, TcpStream},
};
use tokio_udt::UdtConnection;

fn run_progress_fn(config: &Option<impl CoreConfig>, progressing: Progressing) {
    if let Some(config) = config {
        config.run_progress_fn(progressing);
    }
}

pub(crate) async fn send_file<P>(
    udt_connection: &mut UdtConnection,
    path: P,
    handshake_socket: &mut TcpStream,
    config: &Option<ConfigSender<'_>>,
    number_file: u64,
) -> Result<(), UdtError>
where
    P: AsRef<Path> + Sync + Copy,
{
    let handshake = send_handshake_from_file(path, handshake_socket)
        .await
        .map_err(|e| UdtError::Protocol(ProtocolError::Handshake(e)))?;
    let file = File::open(path)
        .await
        .map_err(|e| UdtError::Protocol(ProtocolError::IO(e)))?;
    let mut reader = BufReader::new(file);
    let mut done_bytes = 0;

    let mut buf = vec![0_u8; FBUFFER_SIZE];
    loop {
        let len = reader
            .read(&mut buf)
            .await
            .map_err(|e| UdtError::Protocol(ProtocolError::IO(e)))?;

        if len == 0 {
            break;
        }

        timeout!(udt_connection.send(&buf[0..len]), |_| {
            UdtError::Protocol(ProtocolError::TimeoutExpired)
        })?
        .map_err(|e| UdtError::Protocol(ProtocolError::IO(e)))?;

        done_bytes += len;
        run_progress_fn(
            config,
            Progressing::Yield {
                done_files: number_file,
                total_bytes: handshake.size,
                done_bytes: done_bytes as u64,
                path_to_file: path.as_ref().to_path_buf(),
            },
        );
    }

    run_progress_fn(config, Progressing::Done);
    Ok(())
}

pub(crate) async fn recv_file<P>(
    udt: &mut UdtConnection,
    socket: &mut TcpListener,
    path: P,
    config: &Option<ConfigRecipient<'_>>,
    number_file: u64,
    handshake: Option<Handshake>,
) -> Result<(), UdtError>
where
    P: AsRef<Path> + Sync + Copy,
{
    debug!("raw_recv_file. Getting file");

    // unwrap_or() not working!
    let handshake = if let Some(handshake) = handshake {
        handshake
    } else {
        recv_handshake_from_address(socket)
            .await
            .map_err(|e| UdtError::Protocol(ProtocolError::Handshake(e)))?
    };

    let mut file = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .await
            .map_err(|e| UdtError::Protocol(ProtocolError::IO(e)))?,
    );

    let mut buf = vec![0_u8; NBUFFER_SIZE];
    let mut total_bytes_for_send = handshake.size;
    let mut done_bytes = 0;

    loop {
        let len = udt
            .recv(&mut buf)
            .await
            .map_err(|e| UdtError::Protocol(ProtocolError::ReceivingData(e)))?;

        file.write_all(&buf[0..len])
            .await
            .map_err(|e| UdtError::Protocol(ProtocolError::IO(e)))?;

        total_bytes_for_send -= len as u64;
        done_bytes += len;
        run_progress_fn(
            config,
            Progressing::Yield {
                done_files: number_file,
                total_bytes: handshake.size,
                done_bytes: done_bytes as u64,
                path_to_file: path.as_ref().to_path_buf(),
            },
        );

        if total_bytes_for_send == 0 {
            break;
        }
    }
    file.flush()
        .await
        .map_err(|e| UdtError::Protocol(ProtocolError::IO(e)))?;

    // Check file
    debug!("raw_recv_file. Checking file");
    let mut hasher = get_hasher();
    let hash = file_hashing::get_hash_file(path, &mut hasher)
        .map_err(|e| UdtError::Protocol(ProtocolError::IO(e)))?;

    if hash != handshake.hash {
        debug!(
            "hash not valid! hash: {}; handshake.file_hash: {}",
            hash, handshake.hash
        );
        return Err(UdtError::Protocol(ProtocolError::FileInvalid));
    }

    run_progress_fn(config, Progressing::Done);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;
    use tokio::net::ToSocketAddrs;

    pub(crate) mod detail {
        use super::*;
        use std::net::SocketAddr;
        use tokio_udt::UdtListener;

        pub(crate) async fn async_send(
            address_for_udt: impl ToSocketAddrs,
            address_for_tcp: impl ToSocketAddrs,
            path_to_file: &Path,
        ) -> Result<(), UdtError> {
            let mut udt = UdtConnection::connect(address_for_udt, None)
                .await
                .map_err(|e| UdtError::Protocol(ProtocolError::Connect(e)))?;
            let mut tcp = TcpStream::connect(address_for_tcp)
                .await
                .map_err(|e| UdtError::Protocol(ProtocolError::Connect(e)))?;
            debug!("Done all connect");

            debug!("Running raw_send_file...");
            send_file(&mut udt, path_to_file, &mut tcp, &None, 0).await?;
            debug!("Done raw_send_file!");

            Ok(())
        }

        pub(crate) async fn async_recv(
            address_for_udt: SocketAddr,
            address_for_tcp: impl ToSocketAddrs,
            output: &Path,
        ) -> Result<(), UdtError> {
            let udt_listener = UdtListener::bind(address_for_udt, None)
                .await
                .map_err(|e| UdtError::Protocol(ProtocolError::Bind(e)))?;
            let mut tcp_listener = TcpListener::bind(address_for_tcp)
                .await
                .map_err(|e| UdtError::Protocol(ProtocolError::Bind(e)))?;
            debug!("Done all bind!");

            let (_addr, mut udt_connection) = udt_listener
                .accept()
                .await
                .map_err(|e| UdtError::Protocol(ProtocolError::Accept(e)))?;
            debug!("Accept client: {}", _addr);

            debug!("Running raw_recv_file...");
            recv_file(
                &mut udt_connection,
                &mut tcp_listener,
                output,
                &None,
                0,
                None,
            )
            .await?;
            debug!("Done raw_recv_file!");

            Ok(())
        }
    }

    #[tokio::test]
    async fn udt_raw() {
        crate::init_logger_for_test();

        const ADDRESS_UDT: &str = "127.0.0.1:6432";
        const ADDRESS_TCP: &str = "127.0.0.1:6424";

        let (temp_dir, input_path) = file_hashing::fs::extra::generate_random_file(3626);
        let output_path = temp_dir.join("tess.txt");
        let hash_input = file_hashing::get_hash_file(input_path.path(), &mut get_hasher()).unwrap();

        let (recv, send) = tokio::join!(
            detail::async_recv(
                ADDRESS_UDT.parse().unwrap(),
                ADDRESS_TCP,
                output_path.as_path()
            ),
            detail::async_send(ADDRESS_UDT, ADDRESS_TCP, input_path.path())
        );

        send.unwrap();
        recv.unwrap();
        let hash_output = file_hashing::get_hash_file(output_path, &mut get_hasher()).unwrap();

        assert_eq!(hash_input, hash_output)
    }
}
