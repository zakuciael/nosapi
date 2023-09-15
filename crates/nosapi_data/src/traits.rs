use std::error::Error;
use std::io;
use std::io::{Cursor, Read, Seek};

pub(crate) trait ReadExt: Read {
  fn read_to_utf8_sized(&mut self, size: usize) -> io::Result<String> {
    let mut buf = vec![0u8; size];
    self.read_exact(&mut buf)?;

    Ok(String::from_utf8_lossy(&buf).to_string())
  }
}

pub trait FileParser {
  type Error: Error;

  fn from_bytes<T: AsRef<[u8]> + ?Sized>(bytes: &T) -> Result<Self, Self::Error>
  where
    Self: Sized,
  {
    Self::from_reader(&mut Cursor::new(bytes.as_ref()))
  }

  fn from_reader<T: Read + Seek>(reader: &mut T) -> Result<Self, Self::Error>
  where
    Self: Sized;
}

impl<T> ReadExt for T where T: Read {}
