use std::{path::Path};
use snwf::{prelude::*, protocol::udt::UdtError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errorr {
    #[error("error in udt: {0}")]
    Udt(#[from] snwf::protocol::udt::UdtError),
}

async fn send_file() -> Result<(), Errorr> {
    let mut sender = Sender::new("127.0.0.1".parse().unwrap(), 4324, 6343);
    sender.udt_send_file(Path::new("/home/gladi/test_file.txt")).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    send_file().await?;


    Ok(())
    // TODO
}
