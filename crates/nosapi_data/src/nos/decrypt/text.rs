use std::io;
use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::nos::decrypt::NOSFileDecryptor;

static CRYPTO: [u8; 16] = [
  0x00, 0x20, 0x2D, 0x2E, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x0A, 0x00,
];

#[derive(Default)]
pub struct NOSTextFileSimpleDecryptor {}

impl NOSFileDecryptor for NOSTextFileSimpleDecryptor {
  fn decrypt<T: AsRef<[u8]> + ?Sized>(bytes: &T) -> io::Result<Vec<u8>> {
    if bytes.as_ref().is_empty() {
      return Ok(vec![]);
    }

    let mut reader = Cursor::new(bytes);
    let line_count = reader.read_i32::<LittleEndian>()?;

    let mut result = vec![];
    for _ in 0..line_count {
      let line_length = reader.read_i32::<LittleEndian>()?;
      let mut line_content = vec![0u8; line_length as usize];
      reader.read_exact(&mut line_content)?;

      for byte in line_content.iter() {
        result.push(byte ^ 0x1);
      }

      result.push(b'\n');
    }

    Ok(result)
  }
}

#[derive(Default)]
pub struct NOSTextFileDataDecryptor {}

impl NOSFileDecryptor for NOSTextFileDataDecryptor {
  fn decrypt<T: AsRef<[u8]> + ?Sized>(bytes: &T) -> io::Result<Vec<u8>> {
    let bytes = bytes.as_ref();
    let mut result = vec![];
    let mut pos = 0;

    while pos < bytes.len() {
      let mut current = bytes[pos];
      pos += 1;

      if current == 0xFF {
        result.push(0xD);
        continue;
      }

      let validate = current & 0x7F;

      if (current & 0x80) > 0 {
        for validate in (1..=validate).rev().step_by(2) {
          if pos >= bytes.len() {
            break;
          }

          current = bytes[pos];
          pos += 1;

          result.push(CRYPTO[((current & 0xF0) >> 4) as usize]);
          if validate <= 1 {
            break;
          }

          let second = CRYPTO[(current & 0xF) as usize];
          if second == 0 {
            break;
          }

          result.push(second);
        }
      } else {
        for _ in (1..=validate).rev() {
          if pos >= bytes.len() {
            break;
          }

          current = bytes[pos];
          pos += 1;

          result.push(current ^ 0x33);
        }
      }
    }

    Ok(result)
  }
}
