use crate::blackbox::Blackbox;
use crate::constants::URI_COMPONENT_SET;
use base64::Engine;
use serde::{Serialize, Serializer};

impl Serialize for Blackbox {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let fingerprint_str = {
      let mut buf = Vec::new();
      let mut serializer = serde_json::Serializer::new(&mut buf);
      serde_tuple::SerializeTuple::serialize_tuple(&self.0, &mut serializer).map_err(|err| {
        serde::ser::Error::custom(format!("failed to serialize fingerprint: {}", err))
      })?;

      // It's safe to call `from_utf8_unchecked` since serde_json does the same under the hood
      unsafe { String::from_utf8_unchecked(buf) }
    };

    let url_encoded = percent_encoding::utf8_percent_encode(&fingerprint_str, URI_COMPONENT_SET)
      .collect::<String>()
      .into_bytes();

    let mut gf_encoded = vec![url_encoded[0]];
    for i in 1..url_encoded.len() {
      let a = gf_encoded[i - 1] as u32;
      let b = url_encoded[i] as u32;

      // We can safely call `from_u32_unchecked` since we ensure that the input is a valid Unicode character
      let c = unsafe { std::char::from_u32_unchecked((a + b) % 0x100) };
      gf_encoded.push(c as u8)
    }

    let base64_encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(gf_encoded);

    serializer.serialize_str(&("tra:".to_owned() + &base64_encoded))
  }
}
