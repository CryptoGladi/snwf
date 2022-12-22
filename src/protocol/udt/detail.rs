use super::*;
use tokio::net::{TcpListener, TcpStream};

pub(crate) async fn raw_send_file<P>(
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

pub(crate) async fn raw_recv_file<HashType, P>(
    udt: &mut UdtConnection,
    socket: &mut TcpListener,
    path: P,
) -> Result<(), UdtError>
where
    P: AsRef<Path> + Sync + Copy,
{
    // Send file
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

    pub(crate) mod detail {
    use super::*;

    pub(crate) async fn async_send() -> Result<(), UdtError> {
        let (_temp_dir, child_path) = file_hashing::fs::extra::generate_random_file(325);
        let mut udt = UdtConnection::connect("127.0.0.1:5149", None)
            .await
            .unwrap();
        let mut tcp = TcpStream::connect("127.0.0.1:4174").await.unwrap();

        raw_send_file(&mut udt, child_path.path(), &mut tcp)
            .await
            .unwrap();

        Ok(())
    }

    pub(crate) async fn async_recv() -> Result<(), UdtError> {
        Ok(())
    }
    }



    #[tokio::test]
    async fn udt_raw() {
        //let l = detail::async_recv().await.unwrap();
        //let (send, recv) = tokio::join!(detail::async_send, detail::async_recv);
    }
}
