use crate::blackbox::Blackbox;
use crate::fingerprint::Fingerprint;
use base64::Engine;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};
use serde_tuple_explicit::DeserializeTuple;
use std::fmt::Formatter;

impl<'de> Deserialize<'de> for Blackbox {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct BlackboxVisitor;

    #[allow(clippy::needless_lifetimes)]
    impl<'de> Visitor<'de> for BlackboxVisitor {
      type Value = Blackbox;

      fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "a valid blackbox string")
      }

      fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
      where
        E: Error,
      {
        let blackbox = v.strip_prefix("tra:").unwrap_or(v);
        let base64_decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
          .decode(blackbox)
          .map_err(|e| E::custom(format!("failed to decode base64: {}", e)))?;

        let mut gf_decoded = vec![base64_decoded[0]];
        for i in 1..base64_decoded.len() {
          let mut a = base64_decoded[i] as u32;
          let b = base64_decoded[i - 1] as u32;

          if a < b {
            a += 0x100;
          }

          // We can safely call `from_u32_unchecked` since we ensure that the input is a valid Unicode character
          let c = unsafe { std::char::from_u32_unchecked(a - b) };
          gf_decoded.push(c as u8)
        }

        let url_decoded = percent_encoding::percent_decode(&gf_decoded).collect::<Vec<_>>();
        let fingerprint = {
          let mut deserializer = serde_json::Deserializer::from_slice(&url_decoded);
          Fingerprint::deserialize_tuple(&mut deserializer)
            .map_err(|e| E::custom(format!("failed to deserialize fingerprint: {}", e)))?
        };

        Ok(Blackbox(fingerprint))
      }
    }

    deserializer.deserialize_str(BlackboxVisitor)
  }
}
