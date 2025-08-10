use std::{
  fs::File,
  io::{BufReader, Cursor, Read, Seek},
  path::Path,
};

pub trait ArchiveReader {
  type Error: std::error::Error + From<std::io::Error> + error_stack::Context;

  fn from_reader<R>(reader: &mut R) -> error_stack::Result<Self, Self::Error>
  where
    R: Read + Seek,
    Self: Sized;

  fn from_file<P>(path: P) -> error_stack::Result<Self, Self::Error>
  where
    P: AsRef<Path>,
    Self: Sized,
  {
    let file = File::open(path).map_err(Self::Error::from)?;
    let mut reader = BufReader::new(file);
    Self::from_reader(&mut reader)
  }

  fn from_bytes(data: &[u8]) -> error_stack::Result<Self, Self::Error>
  where
    Self: Sized,
  {
    let mut reader = Cursor::new(data);
    Self::from_reader(&mut reader)
  }
}
