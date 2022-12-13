use std::io::BufRead;

pub(crate) fn progressing_read(
    reader: &mut impl BufRead,
    progress: &mut impl FnMut(usize, &[u8]),
) -> Result<(), std::io::Error> {
    let mut buffer = vec![0u8; 4096];

    loop {
        let len = reader.read(&mut buffer)?;
        progress(len, &buffer[0..len]);

        if len == 0 {
            return Ok(());
        }
    }
}
