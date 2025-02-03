use base64::encoded_len;
use base64::engine::Config;
use sha2::Digest;

/// A helper function for generating a sha512 hash of the Blackbox encryption key.
#[inline]
pub(super) fn create_encryption_key(gsid: &str, account_id: &str) -> Vec<u8> {
  let key = {
    let mut buf = String::with_capacity(gsid.len() + account_id.len() + 1);
    buf.push_str(gsid);
    buf.push('-');
    buf.push_str(account_id);

    buf
  };

  format!("{:x}", sha2::Sha512::digest(&key)).into()
}

/// An XOR cipher implementation reverse-engineered from the original C++ implementation.
pub(super) fn xor(data: &[u8], key: &[u8]) -> Vec<u8> {
  data
    .iter()
    .enumerate()
    .map(|(index, val)| val ^ key[index % key.len()] ^ key[key.len() - (index % key.len()) - 1])
    .collect::<Vec<_>>()
}

/// A thin wrapper around [`base64::Engine::encode_string`] and [`encoded_len`] methods,
/// allowing an ability to handle integer overflows for output capacity.
///
/// ## Returns
/// Returns `false` when output capacity exceeds [`usize::MAX`] otherwise returns `true`
pub(super) fn base64_encode<E, T>(engine: &E, input: T, output_buf: &mut String) -> bool
where
  E: base64::Engine,
  T: AsRef<[u8]>,
{
  let input = input.as_ref();
  let capacity = match encoded_len(input.len(), engine.config().encode_padding()) {
    Some(capacity) => capacity,
    None => return false,
  };

  output_buf.reserve(capacity);
  engine.encode_string(input, output_buf);

  true
}
