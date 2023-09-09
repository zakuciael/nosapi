use std::io;

pub use text::*;

mod text;

pub trait NOSFileDecryptor {
  fn decrypt<T: AsRef<[u8]> + ?Sized>(bytes: &T) -> io::Result<Vec<u8>>;
}
