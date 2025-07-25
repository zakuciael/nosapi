//! Implementation of the `fingerprint` struct found in the `blackbox` string.

use rand::Rng;
pub mod error;

use crate::{fingerprint::error::InvalidGsid, utils::rng_generator, vector::VectorString};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_tuple_explicit::{DeserializeTuple, SerializeTuple};
use serde_with::{
  base64::{Base64, Standard},
  serde_as,
};

/// A `request` struct used when generating an encrypted `blackbox` string.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Request {
  features: Vec<u64>,
  #[serde(rename = "installation")]
  installation_id: String,
  session: String,
}

impl Request {
  /// Create a new `Request` struct from `gsid` and `installation_id`
  ///
  /// # Errors
  /// This method can error whenever the provided `gsid` is invalid.
  pub fn new(gsid: String, installation_id: String) -> Result<Self, InvalidGsid> {
    let features = rng_generator().gen_range(1..9999);
    let session = {
      let index = gsid.rfind("-").ok_or(InvalidGsid)?;
      let session = &gsid[index + 1..];
      session.to_string()
    };

    Ok(Self {
      features: vec![features],
      installation_id,
      session,
    })
  }
}

/// A `fingerprint` struct containing information needed to fingerprint users.
#[serde_as]
#[derive(Serialize, SerializeTuple, Deserialize, DeserializeTuple, Clone, PartialEq, Debug)]
pub struct Fingerprint {
  pub version: u32,
  pub timezone: String,
  pub do_not_track: bool,
  pub browser_engine: String,
  pub os_name: String,
  pub browser_name: String,
  pub vendor: String,
  pub memory: u32,
  pub concurrency: u32,
  pub languages: String,
  pub plugins_hash: String,
  pub gpu: String,
  pub fonts_hash: String,
  pub audio_context_hash: String,
  pub width: u32,
  pub height: u32,
  pub color_depth: u32,
  pub video_codecs_hash: String,
  pub audio_codecs_hash: String,
  pub media_devices_hash: String,
  pub permissions_hash: String,
  pub audio_fingerprint: f64,
  pub webgl_fingerprint: String,
  pub canvas_fingerprint: u32,
  pub creation: DateTime<Utc>,
  pub game: String,
  pub delta: u32,
  pub os_version: Option<String>,
  #[serde_as(as = "Base64<Standard>")]
  pub vector: VectorString,
  pub user_agent: String,
  pub server_time: DateTime<Utc>,
  #[serde(default)]
  pub request: Option<Request>,
}

#[cfg(test)]
mod tests {
  use crate::{
    fingerprint::{Fingerprint, Request},
    vector::VectorString,
  };
  use chrono::DateTime;
  use serde_tuple_explicit::{DeserializeTuple, SerializeTuple};
  use std::str::FromStr;

  #[rstest::fixture]
  //noinspection DuplicatedCode, SpellCheckingInspection
  fn fingerprint_inst() -> Fingerprint {
    Fingerprint {
      version: 9,
      timezone: "Europe/Warsaw".to_string(),
      do_not_track: false,
      browser_engine: "Blink".to_string(),
      os_name: "Linux".to_string(),
      browser_name: "Chrome".to_string(),
      vendor: "Google Inc.".to_string(),
      memory: 8,
      concurrency: 12,
      languages: "en-US,pl,en".to_string(),
      plugins_hash: "f473d473013d58cee78732e974dd4af2e8d0105449c384658cbf1505e40ede50".to_string(),
      gpu: "Google Inc. (AMD),ANGLE (AMD, AMD Radeon RX 5700 XT (radeonsi navi10 LLVM 18.1.8), OpenGL ES 3.2)".to_string(),
      fonts_hash: "6ab3b6cf30d164dd769f1c911cbf6a2fef1e540ecf594b5020f55de0bcfcd844".to_string(),
      audio_context_hash: "d9af7aa1d00f202e8291fe49b9344f69746635eea53e7eace68c10f302cc933a".to_string(),
      width: 1920,
      height: 1080,
      color_depth: 24,
      video_codecs_hash: "3767a83c51cda390de10e37350a640d8be0af56991b5f65b081809a6d29df03f".to_string(),
      audio_codecs_hash: "5a0ef26fd9ff096689feaad0d49fb8551822ea6b3be74a02794c2aa10ead141f".to_string(),
      media_devices_hash: "6aeb6412b24ba7dd08653eb50179026602499917a6400174f9ad7e9bef78abf2".to_string(),
      permissions_hash: "211043d72f4d0b15e2ffab9ecde16f7c4b8e390c7bfb40e72fd2ecd73aa2c3e5".to_string(),
      audio_fingerprint: 124.04347527516074,
      webgl_fingerprint: "d7c53de05c6aea8d6f00d8a17865f81df0a893efb2eb6410bc747bf184234cf3".to_string(),
      canvas_fingerprint: 1640737682,
      creation: FromStr::from_str("2024-12-28T12:56:15.648Z").unwrap(),
      game: "0r397uz4k9n42y0lsaeco3v0utr".to_string(),
      delta: 320,
      os_version: None,
      vector: VectorString::new(
        "jniYEuIsry 5[y,9wS+tK^C'g_`tFmUTHYw|AuQ|IP8&ZAl7uA7TxF_b.Lv8a{i_L/EO? c<KYKRC1p?sPk8o${Y|;>-9<qO'9n7".to_string(),
        DateTime::from_timestamp_millis(1735390575328).unwrap()
      ),
      user_agent: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36".to_string(),
      server_time: FromStr::from_str("2024-12-28T12:56:15.000Z").unwrap(),
      request: None,
    }
  }

  #[rstest::fixture]
  fn fingerprint_json() -> String {
    r#"{
  "version": 9,
  "timezone": "Europe/Warsaw",
  "do_not_track": false,
  "browser_engine": "Blink",
  "os_name": "Linux",
  "browser_name": "Chrome",
  "vendor": "Google Inc.",
  "memory": 8,
  "concurrency": 12,
  "languages": "en-US,pl,en",
  "plugins_hash": "f473d473013d58cee78732e974dd4af2e8d0105449c384658cbf1505e40ede50",
  "gpu": "Google Inc. (AMD),ANGLE (AMD, AMD Radeon RX 5700 XT (radeonsi navi10 LLVM 18.1.8), OpenGL ES 3.2)",
  "fonts_hash": "6ab3b6cf30d164dd769f1c911cbf6a2fef1e540ecf594b5020f55de0bcfcd844",
  "audio_context_hash": "d9af7aa1d00f202e8291fe49b9344f69746635eea53e7eace68c10f302cc933a",
  "width": 1920,
  "height": 1080,
  "color_depth": 24,
  "video_codecs_hash": "3767a83c51cda390de10e37350a640d8be0af56991b5f65b081809a6d29df03f",
  "audio_codecs_hash": "5a0ef26fd9ff096689feaad0d49fb8551822ea6b3be74a02794c2aa10ead141f",
  "media_devices_hash": "6aeb6412b24ba7dd08653eb50179026602499917a6400174f9ad7e9bef78abf2",
  "permissions_hash": "211043d72f4d0b15e2ffab9ecde16f7c4b8e390c7bfb40e72fd2ecd73aa2c3e5",
  "audio_fingerprint": 124.04347527516074,
  "webgl_fingerprint": "d7c53de05c6aea8d6f00d8a17865f81df0a893efb2eb6410bc747bf184234cf3",
  "canvas_fingerprint": 1640737682,
  "creation": "2024-12-28T12:56:15.648Z",
  "game": "0r397uz4k9n42y0lsaeco3v0utr",
  "delta": 320,
  "os_version": null,
  "vector": "am5pWUV1SXNyeSA1W3ksOXdTK3RLXkMnZ19gdEZtVVRIWXd8QXVRfElQOCZaQWw3dUE3VHhGX2IuTHY4YXtpX0wvRU8/IGM8S1lLUkMxcD9zUGs4byR7WXw7Pi05PHFPJzluNyAxNzM1MzkwNTc1MzI4",
  "user_agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
  "server_time": "2024-12-28T12:56:15Z",
  "request": null
}"#.to_string()
  }

  #[rstest::fixture]
  fn fingerprint_array() -> String {
    r#"[
  9,
  "Europe/Warsaw",
  false,
  "Blink",
  "Linux",
  "Chrome",
  "Google Inc.",
  8,
  12,
  "en-US,pl,en",
  "f473d473013d58cee78732e974dd4af2e8d0105449c384658cbf1505e40ede50",
  "Google Inc. (AMD),ANGLE (AMD, AMD Radeon RX 5700 XT (radeonsi navi10 LLVM 18.1.8), OpenGL ES 3.2)",
  "6ab3b6cf30d164dd769f1c911cbf6a2fef1e540ecf594b5020f55de0bcfcd844",
  "d9af7aa1d00f202e8291fe49b9344f69746635eea53e7eace68c10f302cc933a",
  1920,
  1080,
  24,
  "3767a83c51cda390de10e37350a640d8be0af56991b5f65b081809a6d29df03f",
  "5a0ef26fd9ff096689feaad0d49fb8551822ea6b3be74a02794c2aa10ead141f",
  "6aeb6412b24ba7dd08653eb50179026602499917a6400174f9ad7e9bef78abf2",
  "211043d72f4d0b15e2ffab9ecde16f7c4b8e390c7bfb40e72fd2ecd73aa2c3e5",
  124.04347527516074,
  "d7c53de05c6aea8d6f00d8a17865f81df0a893efb2eb6410bc747bf184234cf3",
  1640737682,
  "2024-12-28T12:56:15.648Z",
  "0r397uz4k9n42y0lsaeco3v0utr",
  320,
  null,
  "am5pWUV1SXNyeSA1W3ksOXdTK3RLXkMnZ19gdEZtVVRIWXd8QXVRfElQOCZaQWw3dUE3VHhGX2IuTHY4YXtpX0wvRU8/IGM8S1lLUkMxcD9zUGs4byR7WXw7Pi05PHFPJzluNyAxNzM1MzkwNTc1MzI4",
  "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
  "2024-12-28T12:56:15Z",
  null
]"#.to_string()
  }

  #[rstest::fixture]
  fn request_inst(installation_id: String) -> Request {
    Request {
      features: vec![7734],
      session: "129fae8a8e5c".to_string(),
      installation_id,
    }
  }

  #[rstest::fixture]
  fn gsid() -> String {
    "4fcf4367-1a2e-48b8-9b9a-129fae8a8e5c".to_string()
  }

  #[rstest::fixture]
  fn installation_id() -> String {
    "639edac7-9b6e-454e-80d9-545a5e299860".to_string()
  }

  #[rstest::rstest]
  fn should_correctly_serialize_to_json(fingerprint_inst: Fingerprint, fingerprint_json: String) {
    let res = serde_json::to_string_pretty(&fingerprint_inst);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), fingerprint_json);
  }

  #[rstest::rstest]
  fn should_correctly_serialize_to_array(fingerprint_inst: Fingerprint, fingerprint_array: String) {
    let res = {
      let mut buf = Vec::new();
      let mut serializer = serde_json::Serializer::pretty(&mut buf);

      fingerprint_inst
        .serialize_tuple(&mut serializer)
        .map(|_| unsafe { String::from_utf8_unchecked(buf) })
    };

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), fingerprint_array);
  }

  #[rstest::rstest]
  fn should_correctly_deserialize_from_json(
    fingerprint_json: String,
    fingerprint_inst: Fingerprint,
  ) {
    let res = serde_json::from_str::<Fingerprint>(&fingerprint_json);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), fingerprint_inst);
  }

  #[rstest::rstest]
  fn should_correctly_deserialize_from_array(
    fingerprint_array: String,
    fingerprint_inst: Fingerprint,
  ) {
    let res = {
      let mut deserializer = serde_json::Deserializer::from_str(&fingerprint_array);
      Fingerprint::deserialize_tuple(&mut deserializer)
    };

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), fingerprint_inst);
  }

  #[rstest::rstest]
  fn should_correctly_create_request_struct(
    gsid: String,
    installation_id: String,
    request_inst: Request,
  ) {
    let res = Request::new(gsid, installation_id.clone());
    assert!(res.is_ok());

    assert_eq!(res.unwrap(), request_inst);
  }
}
