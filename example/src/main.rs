use snwf::prelude::*;

#[tokio::main]
async fn main() {
    let mut sender = Sender::new("127.0.0.1".parse().unwrap(), 4324, 6343);
    let mut recipient = Recipient::new("::0".parse().unwrap(), 4324, 6343);

    let (recv, send) = tokio::join!(
        recipient.udt_recv_file(path_output.as_path()),
        sender.udt_send_file(path_input.path())
    );

    send.unwrap();
    recv.unwrap();
    // TODO
}
