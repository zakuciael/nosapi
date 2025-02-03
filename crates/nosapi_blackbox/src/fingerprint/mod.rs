mod support;

use crate::blackbox::Blackbox;
use crate::vector::VectorString;
use chrono::{DateTime, Utc};
use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use serde_tuple_explicit::{DeserializeTuple, SerializeTuple};
use serde_with::serde_as;

#[derive(Constructor, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "builders", derive(bon::Builder))]
pub struct Request {
  pub features: Vec<u64>,
  #[serde(rename = "installation")]
  pub installation_id: String,
  pub session: String,
}

#[serde_as]
#[cfg_attr(feature = "builders", derive(bon::Builder))]
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
  #[serde_as(as = "self::support::Base64")]
  pub vector: VectorString,
  pub user_agent: String,
  pub server_time: DateTime<Utc>,
  #[serde(default)]
  pub request: Option<Request>,
}

impl Fingerprint {
  pub fn into_blackbox(self) -> Blackbox {
    Blackbox::new(self)
  }
}
