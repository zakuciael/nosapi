mod file;

pub use self::TextFile;
pub use file::*;

use crate::{error, helpers::impl_into_iter, traits};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Seek};

#[derive(Debug)]
pub struct TextArchive {
  files: Vec<TextFile>,
}

impl TextArchive {
  pub fn files(&self) -> &[TextFile] {
    &self.files
  }

  pub fn file_count(&self) -> usize {
    self.files.len()
  }

  pub fn find_file_by_id(&self, id: u32) -> Option<&TextFile> {
    self.files.iter().find(|f| f.id() == id)
  }

  pub fn find_file_by_name(&self, name: &str) -> Option<&TextFile> {
    self.files.iter().find(|f| f.name() == name)
  }
}

impl traits::Reader for TextArchive {
  fn from_reader<R: Read + Seek>(reader: &mut R) -> error::Result<Self>
  where
    Self: Sized,
  {
    let file_count = reader.read_u32::<LittleEndian>()?;
    let mut files = Vec::with_capacity(file_count as usize);

    for _ in 0..file_count {
      files.push(TextFile::from_reader(reader)?);
    }

    Ok(Self { files })
  }
}

impl_into_iter! {
  ty: TextArchive,
  iter: Vec<TextFile>,
  item: TextFile,
  expr: |self| self.files.into_iter()
}
