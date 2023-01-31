use snwf::prelude::*;
use std::{path::{Path, PathBuf}, fs::{File, remove_file}};
use thiserror::Error;

const PORT_FOR_SEND_FILES: u16 = 4324;
const PORT_FOR_HANDSHAKE: u16 = 6343;

fn get_input_file() -> PathBuf {
    let mut root = project_root::get_project_root().unwrap();
    root.push("test_file.txt");

    println!("input file in {:?}", root);
    root
}

fn get_output_file() -> PathBuf {
    let mut root = project_root::get_project_root().unwrap();
    root.push("test_file_output.txt");

    println!("output file in {:?}", root);
    root
}

#[derive(Debug, Error)]
pub enum Errorr {
    #[error("error in udt: {0}")]
    Udt(#[from] UdtError),
}

async fn send_file() -> Result<(), Errorr> {
    let mut sender = Sender::new(
        "127.0.0.1".parse().unwrap(),
        PORT_FOR_SEND_FILES,
        PORT_FOR_HANDSHAKE,
    );
    sender
        .udt_send_file(&get_input_file())
        .await?;

    Ok(())
}

async fn recv_file() -> Result<(), Errorr> {
    let mut recipient = Recipient::new(
        "127.0.0.1".parse().unwrap(),
        PORT_FOR_SEND_FILES,
        PORT_FOR_HANDSHAKE,
    );

    if get_output_file().exists() {
        remove_file(get_output_file()).unwrap();
    }

    recipient
        .udt_recv_file(&get_output_file())
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let (recv_future, send_future) = tokio::join!(recv_file(), send_file());

    recv_future.expect("recv error");
    send_future.expect("send error");

    Ok(())
}
