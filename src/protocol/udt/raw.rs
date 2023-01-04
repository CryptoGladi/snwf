//! # Raw [udt](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) implementation

use super::UdtError;
use crate::{
    common::{get_hasher, timeout, Progressing},
    prelude::ConfigSender,
    protocol::handshake::{recv_handshake_from_address, send_handshake_from_file},
};
use core::num;
use log::debug;
use std::path::Path;
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{TcpListener, TcpStream},
};
use tokio_udt::UdtConnection;

fn run_progress_fn(config: &Option<ConfigSender>, progressing: Progressing) {
    if let Some(config) = config {
        if let Some(progress_fn) = config.progress_fn.clone() {
            progress_fn.lock().unwrap()(progressing);
        }
    }
}

pub(crate) async fn send_file<'a, P>(
    udt_connection: &mut UdtConnection,
    path: P,
    handshake_socket: &mut TcpStream,
    config: Option<ConfigSender<'_>>,
    number_file: u64,
) -> Result<(), UdtError>
where
    P: AsRef<Path> + Sync + Copy,
{
    let handshake = send_handshake_from_file(path, handshake_socket).await?;
    let file = File::open(path).await.map_err(UdtError::FileIO)?;
    let mut reader = BufReader::new(file);
    let mut done_bytes = 0;

    let mut buf = vec![0u8; 4096];
    loop {
        let len = reader.read(&mut buf).await.map_err(UdtError::FileIO)?;

        if len == 0 {
            break;
        }

        timeout!(udt_connection.send(&buf[0..len]), |_| {
            UdtError::TimeoutExpired
        })?
        .map_err(UdtError::FileIO)?;

        done_bytes += len;
        run_progress_fn(
            &config,
            Progressing::Yield {
                done_files: number_file,
                total_bytes: handshake.size,
                done_bytes: done_bytes as u64,
            },
        );
    }

    run_progress_fn(&config, Progressing::Done);
    Ok(())
}

pub(crate) async fn recv_file<P>(
    udt: &mut UdtConnection,
    socket: &mut TcpListener,
    path: P,
) -> Result<(), UdtError>
where
    P: AsRef<Path> + Sync + Copy,
{
    // Recv file
    debug!("raw_recv_file. Getting file");

    let handshake = recv_handshake_from_address(socket).await?;
    let mut file = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .await
            .map_err(UdtError::FileIO)?,
    );

    let mut buf = vec![0u8; 4096];
    let mut total_bytes_for_send = handshake.size;

    loop {
        let len = udt.recv(&mut buf).await.map_err(UdtError::NetworkIO)?;

        file.write_all(&buf[0..len])
            .await
            .map_err(UdtError::FileIO)?;

        total_bytes_for_send -= len as u64;
        if total_bytes_for_send == 0 {
            break;
        }
    }
    file.flush().await.map_err(UdtError::FileIO)?;

    // Check file
    debug!("raw_recv_file. Checking file");
    let mut hasher = get_hasher();
    let hash = file_hashing::get_hash_file(path, &mut hasher).map_err(UdtError::FileIO)?;

    if hash != handshake.hash {
        debug!(
            "hash not valid! hash: {}; handshake.file_hash: {}",
            hash, handshake.hash
        );
        return Err(UdtError::FileInvalid);
    }

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
                .map_err(UdtError::Connect)?;
            let mut tcp = TcpStream::connect(address_for_tcp)
                .await
                .map_err(UdtError::Connect)?;
            debug!("Done all connect");

            debug!("Running raw_send_file...");
            send_file(&mut udt, path_to_file, &mut tcp, None, 0).await?;
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
                .map_err(UdtError::Bind)?;
            let mut tcp_listener = TcpListener::bind(address_for_tcp)
                .await
                .map_err(UdtError::Bind)?;
            debug!("Done all bind!");

            let (_addr, mut udt_connection) =
                udt_listener.accept().await.map_err(UdtError::Accept)?;
            debug!("Accept client: {}", _addr);

            debug!("Running raw_recv_file...");
            recv_file(&mut udt_connection, &mut tcp_listener, output).await?;
            debug!("Done raw_recv_file!");

            Ok(())
        }
    }

    #[tokio::test]
    async fn udt_raw() {
        crate::init_logger_for_test();

        const ADDRESS_UDT: &'static str = "127.0.0.1:6432";
        const ADDRESS_TCP: &'static str = "127.0.0.1:6424";

        let (temp_dir, input_path) = file_hashing::fs::extra::generate_random_file(3626);
        let output_path = temp_dir.join("tess.txt");
        let hash_input = file_hashing::get_hash_file(input_path.path(), &mut get_hasher()).unwrap();

        let (recv, send) = tokio::join!(
            detail::async_recv(
                ADDRESS_UDT.parse().unwrap(),
                ADDRESS_TCP,
                output_path.as_path()
            ),
            detail::async_send(ADDRESS_UDT, ADDRESS_TCP, &input_path.path())
        );

        send.unwrap();
        recv.unwrap();
        let hash_output = file_hashing::get_hash_file(output_path, &mut get_hasher()).unwrap();

        assert_eq!(hash_input, hash_output)
    }
}
