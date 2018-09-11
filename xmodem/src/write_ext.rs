use std::io;

pub trait WriteExt: io::Write {
    fn write_max(&mut self, mut buf: &[u8]) -> io::Result<usize> {
        let start_len = buf.len();

        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => break,
                Ok(n) => {
                    let tmp = buf;
                    buf = &tmp[n..];
                }
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }

        self.flush().unwrap();
        Ok(start_len - buf.len())
    }
}

impl<T: io::Write> WriteExt for T {}
