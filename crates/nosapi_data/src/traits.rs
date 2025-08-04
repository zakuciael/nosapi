use crate::error;
use std::{
  fs::File,
  io::{BufReader, BufWriter, Cursor, Read, Seek, Write},
  path::Path,
};

pub trait Reader {
  fn from_reader<R>(reader: &mut R) -> error::Result<Self>
  where
    R: Read + Seek,
    Self: Sized;

  fn from_file<P>(path: P) -> error::Result<Self>
  where
    P: AsRef<Path>,
    Self: Sized,
  {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    Self::from_reader(&mut reader)
  }

  fn from_bytes(data: &[u8]) -> error::Result<Self>
  where
    Self: Sized,
  {
    Self::from_reader(&mut Cursor::new(data))
  }
}

pub trait Writer {
  fn to_writer<W: Write + Seek>(self, writer: &mut W) -> error::Result<()>
  where
    Self: Sized;

  fn to_file<P>(self, path: P) -> error::Result<()>
  where
    P: AsRef<Path>,
    Self: Sized,
  {
    let mut writer = BufWriter::new(File::create(path)?);
    self.to_writer(&mut writer)?;
    writer.flush()?;

    Ok(())
  }

  fn to_bytes(self) -> error::Result<Vec<u8>>
  where
    Self: Sized,
  {
    let mut buf = Cursor::new(Vec::new());
    self.to_writer(&mut buf)?;

    Ok(buf.into_inner())
  }
}
