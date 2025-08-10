mod error;
mod file;

use super::traits;
use crate::macros::read;
use error_stack::ResultExt;
use std::io::{Read, Seek};

pub use error::ArchiveError;
pub use file::File;

#[derive(Debug)]
pub struct Archive {
  files: Vec<File>,
}

impl Archive {
  pub fn files(&self) -> &[File] {
    &self.files
  }
}

impl traits::ArchiveReader for Archive {
  type Error = ArchiveError;

  fn from_reader<R>(reader: &mut R) -> error_stack::Result<Self, Self::Error>
  where
    R: Read + Seek,
    Self: Sized,
  {
    let file_count = read!(reader, file_count, u32).change_context(ArchiveError)?;
    let mut files = Vec::with_capacity(file_count as usize);

    for _ in 0..file_count {
      files.push(File::read(reader).change_context(ArchiveError)?);
    }

    Ok(Self { files })
  }
}
