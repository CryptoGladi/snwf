# snwf

> Library for simple network work on files

![license](https://img.shields.io/github/license/CryptoGladi/snwf?style=for-the-badge) ![buld](https://img.shields.io/github/actions/workflow/status/CryptoGladi/snwf/.github/workflows/rust.yml?branch=master&style=for-the-badge)

# Motivation :rocket:

If you just need to transfer a file over the network to another computer,
but you don't want to write hundreds of lines of code to implement a
**receiver** and **sender**, then this library is right for you.

# Features :star:

*   **udt** - enable [udt](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) protocol support

# Example (udt)

```rust
use snwf::prelude::*;
use std::path::Path;

#[tokio::main]
async fn main() {
   let mut sender = Sender::new("127.0.0.1".parse().unwrap(), 4324, 6343, None);
   let mut recipient = Recipient::new("::0".parse().unwrap(), 4324, 6343, None);

   let (recv, send) = tokio::join!(
       recipient.udt_recv_file(Path::new("other_file.txt")),
       sender.udt_send_file(Path::new("file_for_send.txt"))
   );
   
   send.unwrap();
   recv.unwrap();
}
```
