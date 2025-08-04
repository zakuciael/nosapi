use crate::{error, traits};
use byteorder::{LittleEndian, ReadBytesExt};
use encoding_rs::Encoding;
use lazy_static::lazy_static;
use regex::Regex;
use size::Size;
use std::{
  cell::OnceCell,
  io::{Cursor, Read, Seek},
};

lazy_static! {
  static ref ENCODING_DETECTION_REGEX: Regex =
    Regex::new(r"^_code_(\w{2})_\w*\.txt$").expect("Invalid regex pattern for encoding detection");
}

#[derive(derive_more::Debug, derive_more::Display)]
#[display("{}", self.content())]
pub struct TextFile {
  id: u32,
  name: String,
  // FIXME: Implement writing archive files
  #[debug(skip)]
  _file_type: TextFileType,
  #[debug(skip)]
  encoding: &'static Encoding,
  #[debug("{}", Size::from_bytes(*file_size))]
  file_size: u32,
  #[debug("{:?}", self.content())]
  content: OnceCell<String>,
  #[debug(skip)]
  raw_content: Vec<u8>,
}

impl TextFile {
  pub fn id(&self) -> u32 {
    self.id
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn size(&self) -> u32 {
    self.file_size
  }

  pub fn content(&self) -> &str {
    self
      .content
      .get_or_init(|| self.encoding.decode(&self.raw_content).0.into_owned())
  }

  pub fn raw(&self) -> &[u8] {
    &self.raw_content
  }

  pub fn encoding(&self) -> &'static Encoding {
    self.encoding
  }
}

impl traits::Reader for TextFile {
  fn from_reader<R: Read + Seek>(reader: &mut R) -> error::Result<Self>
  where
    Self: Sized,
  {
    let id = reader.read_u32::<LittleEndian>()?;

    let name = {
      let size = reader.read_u32::<LittleEndian>()?;
      let mut buf = vec![0x0; size as usize];
      reader.read_exact(&mut buf)?;

      String::from_utf8_lossy(&buf).into_owned()
    };

    let file_type = {
      let raw = reader.read_u32::<LittleEndian>()?;

      match raw {
        0 => TextFileType::LST,
        1 => TextFileType::DAT,
        unknown => TextFileType::Unknown(unknown),
      }
    };

    let file_size = reader.read_u32::<LittleEndian>()?;
    let encoding = get_encoding(&name);

    let content = {
      let mut buf = vec![0x0; file_size as usize];
      reader.read_exact(&mut buf)?;

      file_type.decrypt(buf)?
    };

    Ok(Self {
      id,
      name,
      _file_type: file_type,
      encoding,
      file_size,
      content: Default::default(),
      raw_content: content,
    })
  }
}

#[allow(clippy::upper_case_acronyms)]
#[allow(dead_code)]
#[derive(Debug)]
enum TextFileType {
  DAT,
  LST,
  Unknown(u32),
}

impl TextFileType {
  const DECRYPT_ARRAY: [u8; 16] = [
    0x00, 0x20, 0x2D, 0x2E, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x0A, 0x00,
  ];

  pub fn decrypt(&self, data: Vec<u8>) -> error::Result<Vec<u8>> {
    match self {
      TextFileType::DAT => self.decrypt_dat(data),
      TextFileType::LST => self.decrypt_lst(data),
      TextFileType::Unknown(_) => Ok(data),
    }
  }

  fn decrypt_lst(&self, data: Vec<u8>) -> error::Result<Vec<u8>> {
    let mut reader = Cursor::new(&data);
    let line_count = reader.read_u32::<LittleEndian>()?;

    // Reserve at least `data.len()` amount of bytes + 1 byte per line for a new line character.
    let mut result: Vec<u8> = Vec::with_capacity(data.len() + line_count as usize);

    for _ in 0..line_count {
      let line_length = reader.read_u32::<LittleEndian>()?;
      let mut line_data = vec![0x0; line_length as usize];
      reader.read_exact(&mut line_data)?;

      // XOR decode the line and add a newline
      result.extend(
        line_data
          .into_iter()
          .map(|b| b ^ 0x1)
          .chain(std::iter::once(b'\n')),
      );
    }

    Ok(result)
  }

  fn decrypt_dat(&self, data: Vec<u8>) -> error::Result<Vec<u8>> {
    // Reserve at least 2x more bytes then the `data.len()`
    let mut result: Vec<u8> = Vec::with_capacity(data.len() * 2);
    let mut index = 0;

    while let Some(&byte) = data.get(index) {
      index += 1;

      if byte == 0xFF {
        result.push(b'\r');
        continue;
      }

      let byte_count = byte & 0x7F;
      let is_compressed = byte & 0x80 != 0;

      index = if is_compressed {
        self.process_compressed_data(&data, index, byte_count, &mut result)
      } else {
        self.process_uncompressed_data(&data, index, byte_count, &mut result)
      }
    }

    Ok(result)
  }

  fn process_compressed_data(
    &self,
    data: &[u8],
    mut index: usize,
    mut byte_count: u8,
    result: &mut Vec<u8>,
  ) -> usize {
    while byte_count > 0 && index < data.len() {
      let byte = data[index];
      index += 1;

      let first_byte = Self::DECRYPT_ARRAY[((byte & 0xF0) >> 4) as usize];
      result.push(first_byte);
      byte_count -= 1;

      if byte_count > 0 {
        let second_byte = Self::DECRYPT_ARRAY[(byte & 0xF) as usize];

        if second_byte == 0 {
          break;
        }

        result.push(second_byte);
        byte_count -= 1;
      }
    }

    index
  }

  fn process_uncompressed_data(
    &self,
    data: &[u8],
    index: usize,
    byte_count: u8,
    result: &mut Vec<u8>,
  ) -> usize {
    let end_index = (index + byte_count as usize).min(data.len());

    result.extend(data[index..end_index].iter().map(|&b| b ^ 0x33));
    end_index
  }
}

fn get_encoding(file_name: &str) -> &'static Encoding {
  ENCODING_DETECTION_REGEX
    .captures(file_name)
    .and_then(|caps| caps.get(1))
    .map(|m| match m.as_str() {
      "de" | "pl" | "it" | "cz" => encoding_rs::WINDOWS_1250,
      "ru" => encoding_rs::WINDOWS_1251,
      "uk" | "fr" | "es" => encoding_rs::WINDOWS_1252,
      "tr" => encoding_rs::WINDOWS_1254,
      "hk" | "tw" => encoding_rs::BIG5,
      _ => encoding_rs::WINDOWS_1250,
    })
    .unwrap_or(encoding_rs::EUC_KR)
}
