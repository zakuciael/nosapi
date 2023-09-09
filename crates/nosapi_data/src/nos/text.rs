use std::io::{Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};
use derivative::Derivative;
use time::OffsetDateTime;

use crate::exts::{FromExt, ReadExt};
use crate::nos::decrypt::{NOSFileDecryptor, NOSTextFileDataDecryptor, NOSTextFileSimpleDecryptor};
use crate::nos::error::{Error, Result};
use crate::nos::NOSFileType;

static OLE_TIME_CHECK: [u8; 4] = [0xEE, 0x3E, 0x32, 0x01];

#[derive(Debug, PartialEq, Hash)]
pub struct NOSTextFile {
  pub file_count: i32,
  pub files: Vec<NOSTextFileEntry>,
  pub ole_time: Option<OffsetDateTime>,
}

#[derive(Derivative, PartialEq, Hash)]
#[derivative(Debug)]
pub struct NOSTextFileEntry {
  pub file_number: i32,
  pub name: String,
  pub is_dat: bool,
  pub size: i32,
  #[derivative(Debug = "ignore")]
  pub raw_content: Vec<u8>,
}

impl FromExt for NOSTextFile {
  type Error = Error;

  fn from_reader<T: Read + Seek>(reader: &mut T) -> Result<Self>
  where
    Self: Sized,
  {
    let file_type = NOSFileType::from_reader(reader)?;

    if file_type != NOSFileType::Text {
      return Err(Error::InvalidFileType {
        expected: NOSFileType::Text,
        received: file_type,
      });
    }

    reader.seek(SeekFrom::Start(0))?;
    let file_count: i32 = reader.read_i32::<LittleEndian>()?;
    let mut files = Vec::with_capacity(file_count as usize);

    for _ in 0..file_count {
      files.push(NOSTextFileEntry::from_reader(reader)?);
    }

    let mut ole_time = [0u8; 12];
    let ole_time = if reader.read(&mut ole_time)? == 12 && ole_time[8..12] == OLE_TIME_CHECK {
      let mut buf: [u8; 8] = [0u8; 8];
      buf.copy_from_slice(&ole_time[0..8]);

      let variant = f64::from_le_bytes(buf);
      let unix_time = (-2208988800f64 + ((variant - 2.00001) * 86400f64)) as i64;

      Some(OffsetDateTime::from_unix_timestamp(unix_time)?)
    } else {
      None
    };

    Ok(Self {
      file_count,
      files,
      ole_time,
    })
  }
}

impl FromExt for NOSTextFileEntry {
  type Error = Error;

  fn from_reader<T: Read + Seek>(reader: &mut T) -> Result<Self>
  where
    Self: Sized,
  {
    let file_number = reader.read_i32::<LittleEndian>()?;
    let name_size = reader.read_i32::<LittleEndian>()? as usize;
    let name = reader.read_to_utf8_sized(name_size)?;
    let is_dat = reader.read_i32::<LittleEndian>()? > 0;
    let file_size = reader.read_i32::<LittleEndian>()?;

    let mut content = vec![0u8; file_size as usize];
    reader.read_exact(&mut content)?;

    let content = if is_dat {
      NOSTextFileDataDecryptor::decrypt(&content)?
    } else {
      NOSTextFileSimpleDecryptor::decrypt(&content)?
    };

    Ok(NOSTextFileEntry {
      file_number,
      name,
      is_dat,
      size: file_size,
      raw_content: content,
    })
  }
}
