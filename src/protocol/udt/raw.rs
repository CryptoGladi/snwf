use super::UdtError;
use crate::{
    common::{get_hasher, TIMEOUT},
    protocol::handshake::{recv_handshake_from_address, send_handshake_from_file},
};
use log::debug;
use std::path::Path;
use tokio::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter},
    net::{TcpListener, TcpStream},
    time::timeout,
};
use tokio_udt::UdtConnection;

pub(crate) async fn send_file<P>(
    udt: &mut UdtConnection,
    path: P,
    socket: &mut TcpStream,
) -> Result<(), UdtError>
where
    P: AsRef<Path> + Sync + Copy,
{
    send_handshake_from_file(path, socket).await?;
    let file = File::open(path).await.map_err(UdtError::FileIO)?;
    let mut reader = BufReader::new(file);

    tokio::io::copy(&mut reader, udt)
        .await
        .map_err(UdtError::FileIO)?;

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

    if let Ok(result) = timeout(TIMEOUT, tokio::io::copy(udt, &mut file)).await {
        result.map_err(UdtError::FileIO)?;
        return Ok(());
    };
    drop(file);

    // Check file
    debug!("raw_recv_file. Checking file");
    let mut hasher = get_hasher();
    let hash = file_hashing::get_hash_file(path, &mut hasher).map_err(UdtError::FileIO)?;

    if hash != handshake.file_hash {
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
        use std::{net::SocketAddr, path::PathBuf};
        use tokio_udt::UdtListener;

        pub(crate) async fn async_send(
            address_for_udt: impl ToSocketAddrs,
            address_for_tcp: impl ToSocketAddrs,
            path_to_file: &Path
        ) -> Result<(), UdtError> {
            let mut udt = UdtConnection::connect(address_for_udt, None)
                .await
                .map_err(UdtError::Connect)?;
            let mut tcp = TcpStream::connect(address_for_tcp)
                .await
                .map_err(UdtError::Connect)?;
            debug!("Done all connect");

            debug!("Running raw_send_file...");
            send_file(&mut udt, path_to_file, &mut tcp).await?;
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
            recv_file(
                &mut udt_connection,
                &mut tcp_listener,
                output,
            )
            .await?;
            debug!("Done raw_recv_file!");

            Ok(())
        }
    }

    #[tokio::test]
    async fn udt_raw() {
        const ADDRESS_UDT: &'static str = "127.0.0.1:6432";
        const ADDRESS_TCP: &'static str = "127.0.0.1:6424";

        let (temp_dir, input_path) = file_hashing::fs::extra::generate_random_file(3626);
        let output_path = temp_dir.join("tess.txt");
        let hash_input = file_hashing::get_hash_file(&input_path, &mut get_hasher()).unwrap();

        let (send, recv) = tokio::join!(
            detail::async_recv(ADDRESS_UDT.parse().unwrap(), ADDRESS_TCP, output_path.as_path()),
            detail::async_send(ADDRESS_UDT, ADDRESS_TCP, input_path.path())
        );

        send.unwrap();
        recv.unwrap();
        let hash_output = file_hashing::get_hash_file(output_path, &mut get_hasher()).unwrap();

        assert_eq!(hash_input, hash_output)
    }
}
