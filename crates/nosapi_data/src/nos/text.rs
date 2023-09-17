use std::io::{Cursor, Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};
use chrono::{DateTime, TimeZone, Utc};
use derivative::Derivative;

use crate::nos::decrypt::{NOSFileDecryptor, NOSTextFileDataDecryptor, NOSTextFileSimpleDecryptor};
use crate::nos::error::NOSFileError;
use crate::nos::NOSFileType;
use crate::traits::ReadExt;

static OLE_TIME_CHECK: [u8; 4] = [0xEE, 0x3E, 0x32, 0x01];

/// Represents a parsed `.NOS` text file.
///
/// # Examples
/// Here is an example of loading the `NSgtdData.NOS`
/// using the [`memmap2`](https://crates.io/crates/memmap2) crate
/// ```
/// use std::error::Error;
/// use std::fs::File;
/// use std::io::Read;
/// use nosapi_data::nos::NOSTextFile;
/// use memmap2::Mmap;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///   let file = File::open("./tests/fixtures/NSgtdData.NOS")?;
///   let mmap = unsafe { Mmap::map(&file)? };
///   let result = NOSTextFile::from_bytes(&mmap)?;
///
///   println!("{:?}", result);
///   Ok(())
/// }
/// ```
#[derive(Clone, Eq, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct NOSTextFile {
  /// The number of file entries contained in the `.NOS` file.
  pub file_count: i32,
  /// A vector of [`NOSTextFileEntry`] structs, representing the file entries within the `.NOS` file.
  pub files: Vec<NOSTextFileEntry>,
  /// The last file modification timestamp, if available.
  pub last_modified_date: Option<DateTime<Utc>>,
}

/// Represents an file entry within a `.NOS` text file.
#[derive(Clone, Eq, Derivative, PartialEq, Hash)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derivative(Debug)]
pub struct NOSTextFileEntry {
  /// Stores the file number, that indicates the position of this file inside the file list.
  pub file_number: i32,
  /// Represents the name of this file.
  pub name: String,
  /// Indicates whether the file is encrypted as `.dat` file or not.
  pub is_dat: bool,
  /// Stores the size of the file in bytes.
  pub size: i32,
  /// Decrypted raw content of the file
  #[derivative(Debug = "ignore")]
  pub raw_content: Vec<u8>,
}

impl NOSTextFile {
  /// Creates a [`NOSTextFile`] instance from a byte slice
  ///
  /// # Errors
  /// - [`InvalidFileHeader`](NOSFileError::InvalidFileHeader) - An invalid file header was detected.
  /// - [`InvalidFileType`](NOSFileError::InvalidFileType) - The file is of a wrong file type.
  /// - [`IO`](NOSFileError::IO) - An error was encountered while reading the file.
  pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(bytes: &T) -> Result<Self, NOSFileError> {
    Self::from_reader(&mut Cursor::new(bytes.as_ref()))
  }

  /// Creates a [`NOSTextFile`] instance from a reader implementing [`Read`] and [`Seek`] traits.
  ///
  /// # Errors
  /// - [`InvalidFileHeader`](NOSFileError::InvalidFileHeader) - An invalid file header was detected.
  /// - [`InvalidFileType`](NOSFileError::InvalidFileType) - The file is of a wrong file type.
  /// - [`IO`](NOSFileError::IO) - An error was encountered while reading the file.
  pub fn from_reader<T: Read + Seek>(reader: &mut T) -> Result<Self, NOSFileError> {
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

    let mut raw_ole_time = [0u8; 12];
    let last_modified_date =
      if reader.read(&mut raw_ole_time)? == 12 && raw_ole_time[8..12] == OLE_TIME_CHECK {
        let variant = &raw_ole_time[0..8].try_into().map(f64::from_le_bytes).ok();

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
      last_modified_date,
    })
  }
}

impl NOSTextFileEntry {
  /// Creates a [`NOSTextFileEntry`] instance from a byte slice.
  ///
  /// # Errors
  /// This method can return an [`IO`](NOSFileError::IO) error if it fails to read the file.
  pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(bytes: &T) -> Result<Self, NOSFileError> {
    Self::from_reader(&mut Cursor::new(bytes.as_ref()))
  }

  /// Creates a [`NOSTextFileEntry`] instance from a reader implementing [`Read`] trait.
  ///
  /// # Errors
  /// This method can return an [`IO`](NOSFileError::IO) error if it fails to read the file.
  pub fn from_reader<T: Read>(reader: &mut T) -> Result<Self, NOSFileError> {
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
