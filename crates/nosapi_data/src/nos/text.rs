use std::io::{Cursor, Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};
use chrono::{DateTime, TimeZone, Utc};
use derivative::Derivative;

use crate::nos::decrypt::{NOSFileDecryptor, NOSTextFileDataDecryptor, NOSTextFileSimpleDecryptor};
use crate::nos::error::NOSFileError;
use crate::nos::NOSFileType;
use crate::traits::ReadExt;

static OLE_TIME_CHECK: [u8; 4] = [0xEE, 0x3E, 0x32, 0x01];

#[derive(Clone, Eq, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct NOSTextFile {
  pub file_count: i32,
  pub files: Vec<NOSTextFileEntry>,
  pub ole_time: Option<DateTime<Utc>>,
}

#[derive(Clone, Eq, Derivative, PartialEq, Hash)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derivative(Debug)]
pub struct NOSTextFileEntry {
  pub file_number: i32,
  pub name: String,
  pub is_dat: bool,
  pub size: i32,
  #[derivative(Debug = "ignore")]
  pub raw_content: Vec<u8>,
}

impl NOSTextFile {
  pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(bytes: &T) -> Result<Self, NOSFileError> {
    Self::from_reader(&mut Cursor::new(bytes.as_ref()))
  }

  fn from_reader<T: Read + Seek>(reader: &mut T) -> Result<Self, NOSFileError> {
    let file_type = NOSFileType::from_reader(reader)?;

    if file_type != NOSFileType::Text {
      return Err(NOSFileError::InvalidFileType {
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
      let variant = &ole_time[0..8].try_into().map(f64::from_le_bytes).ok();

      variant
        .map(|variant| {
          let unix_timestamp = (-2208988800f64 + ((variant - 2.00001) * 86400f64)) as i64;
          Utc.timestamp_opt(unix_timestamp, 0).single()
        })
        .unwrap_or(None)
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

impl NOSTextFileEntry {
  pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(bytes: &T) -> Result<Self, NOSFileError> {
    Self::from_reader(&mut Cursor::new(bytes.as_ref()))
  }

  pub fn from_reader<T: Read + Seek>(reader: &mut T) -> Result<Self, NOSFileError> {
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
