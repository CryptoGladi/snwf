# snwf

> Library for simple network work on files

[![codacy badge](https://img.shields.io/codacy/grade/187d6864d2684ec7bae17e2ad1277f67?style=for-the-badge)](https://www.codacy.com/gh/CryptoGladi/snwf/dashboard?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=CryptoGladi/snwf&amp;utm_campaign=Badge_Grade) ![total lines](https://img.shields.io/tokei/lines/github/CryptoGladi/snwf?style=for-the-badge) ![repo size](https://img.shields.io/github/repo-size/CryptoGladi/snwf?style=for-the-badge) ![license](https://img.shields.io/github/license/CryptoGladi/snwf?style=for-the-badge)

# Motivation :rocket:

If you just need to transfer a file over the network to another computer,
but you don't want to write hundreds of lines of code to implement a
**receiver** and **sender**, then this library is right for you.

# Features :star:

* **udt** - enable [udt](https://en.wikipedia.org/wiki/UDP-based_Data_Transfer_Protocol) protocol support

# Example (udt)

```rust
use snwf::prelude::*;
use std::path::Path;

#[tokio::main]
async fn main() {
   let mut sender = Sender::new("127.0.0.1".parse().unwrap(), 4324, 6343);
   let mut recipient = Recipient::new("::0".parse().unwrap(), 4324, 6343);

   let (recv, send) = tokio::join!(
       recipient.udt_recv_file(Path::new("other_file.txt")),
       sender.udt_send_file(Path::new("file_for_send.txt"))
   );
   
   send.unwrap();
   recv.unwrap();
}
```

TODO Add badges
