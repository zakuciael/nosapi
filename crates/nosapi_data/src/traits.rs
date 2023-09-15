use std::io;
use std::io::Read;

pub(crate) trait ReadExt: Read {
  fn read_to_utf8_sized(&mut self, size: usize) -> io::Result<String> {
    let mut buf = vec![0u8; size];
    self.read_exact(&mut buf)?;

    Ok(String::from_utf8_lossy(&buf).to_string())
  }
}

impl<T> ReadExt for T where T: Read {}
