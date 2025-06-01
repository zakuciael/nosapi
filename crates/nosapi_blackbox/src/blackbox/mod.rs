//! Implementation of the `blackbox` string.

mod de;
pub mod error;
mod ser;

use self::error::{DecryptBlackboxError, EncryptBlackboxError};
use crate::fingerprint::Fingerprint;
use base64::Engine;
use sha2::Digest;

fn create_encryption_key(gsid: String, account_id: String) -> Vec<u8> {
  let key = format!("{}-{}", gsid, account_id);
  let hash = sha2::Sha512::digest(&key);

  format!("{:x}", hash).into()
}

fn xor(data: &[u8], key: &[u8]) -> Vec<u8> {
  data
    .iter()
    .enumerate()
    .map(|(index, val)| val ^ key[index % key.len()] ^ key[key.len() - (index % key.len()) - 1])
    .collect::<Vec<_>>()
}

/// A struct defining methods for working with the `blackbox` string
///
/// # Examples
/// ## De/serializing an unencrypted `blackbox` string
/// ```rust
/// use nosapi_blackbox::blackbox::Blackbox;
/// let encoded_blackbox = "tra:JVqc1fosb5TG-D2yJJMDaI2_BVy9L6IDep_RAyhanQNk0EOozf9CZ5nLDXniULvgEkRpm94DNWezHIr_d5zOACVXmr_xI2bOQK8cgabYCi9hpMn7LXTjUrkliq_hEVrIK1l-sOIHOXy02QtOf7HWCEtwotQ5p9QpfKHTFobyF0mM8V-EtugNP4Kn2QtxpdwPc6feEUFypQk-dtk-o9oSSXyuE0yDtxt_sxR6rBFJrd0OPnOn2xR3quIWTIG5HH7kFUp6rxRIeN1BptsLMGKUuesuU4W3_m3cQ68UOWub5FK14wg6apLTIGSNsuQnaLb9SY6z5RU9fssPNGapzgAwcb4CJ1mJ2zygBXTiBzlpuxM4aprPBjZmi73tRZm-8CBIuht_5FPBNJ3C9CSS82nSAzNYiroGUqj1Gkx8reUTRHKq0_gqbZLE9EOzGIbNGT5woOU4XY-_8iBSe6DSBClbnsP1J12-IFO160605xd7rOIWet4VS4TqG3636Bl83kR62w1z2D5v1Ak9bdI1m9AJPZ_UBDZmzAE2mv8vkfRavSFZjcHmGEpvoeQJO23RCmvRCGnK-1-PvyVXh7keVojB8li98SqMxfgsYMb8NWyg1gw_dNk-n9QHbKMIacwxZ58CM2PJ_CxewSRdkMMkSXut0gRHeLHjEzhqrd4ORnabzRBCdpvNEDVnmcwDOXDRCTyf1AVozC1gmcktksPzWIvC9Spau_ElVbnxU7joSa_kGlOMvR9UuvAlh7fvIFiIwSJYvO4ni_EhVLrfEUNomt0CNGab_CyR9ylfxSliyC5el80DO3TaP6ABZZX5LWbMLmab0AE5a50CY5n7LpD1LGDB8SNak8cqXL0eT3_kRanaDj-lyvwuU4XI7R9Rh-hNr-UZSnzeEESmBz6iBjZupNkMcdMIOGmg2Qk7cafXCT12r-gZULHnG0t7rOMXfbYXe7IXULIXfbTsTa8VR2ye0PUnao_B8yVWh7frHoK561GF6Rl7rOFGeN5EpQdApQhs0QI4ntU4bM4Ga57XB2qhA2nL_y-Uy_1jx_lewSVcj_BRg-YZfrPYCjxhk9YHOW2by_8yZp3SBDtwodcHPnKXyQwxY5X5MJPI-1_E9CmMwiOI6SGFuyFRgeUdfq_mHlSJ7ydYvCJSs-skV7wihLYbfbPnGEiqDUR4rxF3qOAURnmtEHapzgAyV4nM_TNnl84BOG6m2P0vcpfJ-y1dj8PwIVOAsuo-b6HG-Tpvpcr9Pm-k0gg8dM7zJVd8rvEWSHqqHE-IvzSu4k2G9Cha0wNv4kOoC3qtI1PIPK7TBTdcjtEENmaLvQBu40-74BJVeqzeP6zhUaj9U4TXL332W67vIHeqFYjXL5PnMmW3A1vGE4HbDEWsEFWvI3nPIWrBGX21Bl60BmyxHW69AFq7DGPaDXHGCz6U3ESL4xVe0ydvyPxVrSGR6RmQBlit5Qo8gssSX5fqG4fTKJPgWLv_OLIHTsH1V9AiWbAIf7YGb5_UJGyyAkzGMqf1bq8nde88bbo0nxZkuBtMmRNckLXnGT5ws9gKPIn4cttHsxQ5a7HmFERpm8vzS3yt0gVHbJ7OGoPxZt4DNWXdFUuq4BQ9YpTEBXXlUbYNctQfiPwhU5nOAThmmc_0JlZ-yRFlsv4jVZi97x-L9F_E6RtLkvdaxTRdgrTkJ48BcN1CZ5nfEEN0otIAMF6Os-UVaMkvkAJrkMIIPXCn1Qg-Y5XH7B5hhrjqHEx-st8QQm-h2S1ekLXoKV6UuewtXpPtEkR2m80QfvNfy_AlaQ";
///
/// // Decode
/// let blackbox = Blackbox::decode(encoded_blackbox.to_string()).unwrap();
/// // Encode
/// let re_encoded_blackbox = blackbox.encode().unwrap();
///
/// assert_eq!(encoded_blackbox, re_encoded_blackbox);
/// ```
/// ## Decrypt/Encrypt a `blackbox` string
/// ```rust
/// use nosapi_blackbox::blackbox::Blackbox;
/// let encrypted_blackbox = "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";
/// let gsid = "4fcf4367-1a2e-48b8-9b9a-129fae8a8e5c";
/// let account_id = "67e55044-10b1-426f-9247-bb680e5fe0c8";
///
/// // Decrypt
/// let blackbox = Blackbox::decrypt(encrypted_blackbox.to_string(), gsid.to_string(), account_id.to_string()).unwrap();
/// // Encrypt
/// let re_encrypted_blackbox = blackbox.encrypt(gsid.to_string(), account_id.to_string()).unwrap();
///
/// assert_eq!(encrypted_blackbox, re_encrypted_blackbox);
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct Blackbox(pub Fingerprint);

impl Blackbox {
  pub fn new(fingerprint: Fingerprint) -> Self {
    Self(fingerprint)
  }

  /// Decode an encoded `blackbox` string.
  ///
  /// # Errors
  /// This method can error whenever the deserialization of the `Blackbox` struct fails.
  pub fn decode(value: String) -> Result<Self, serde_plain::Error> {
    serde_plain::from_str::<Blackbox>(&value)
  }

  /// Encode a `blackbox` string
  ///
  /// # Errors
  /// This method can error whenever the serialization of the `Blackbox` struct fails.
  pub fn encode(&self) -> Result<String, serde_plain::Error> {
    serde_plain::to_string(self)
  }

  /// Decrypt a `blackbox` string using the provided `gsid` and `account_id`.
  ///
  /// # Errors
  /// This method can error whenever one of the following scenarios occurred:
  /// - The input is an invalid `base64` string.
  /// - The decryption produced an invalid utf-8 string.
  /// - The decryption produced an invalid `Blackbox` struct.
  pub fn decrypt(
    value: String,
    gsid: String,
    account_id: String,
  ) -> Result<Blackbox, DecryptBlackboxError> {
    let base64_decoded = base64::engine::general_purpose::STANDARD.decode(value)?;
    let encryption_key = create_encryption_key(gsid, account_id);

    let xored = xor(&base64_decoded, &encryption_key);
    let blackbox = String::from_utf8(xored)?;
    Ok(serde_plain::from_str::<Blackbox>(&blackbox)?)
  }

  /// Encrypt the `blackbox` string using the provided `gsid` and `account_id`.
  ///
  /// # Errors
  /// This method can error whenever the serialization of the `Blackbox` struct fails.
  pub fn encrypt(self, gsid: String, account_id: String) -> Result<String, EncryptBlackboxError> {
    let blackbox = serde_plain::to_string(&self)?.into_bytes();
    let encryption_key = create_encryption_key(gsid, account_id);

    let xored = xor(&blackbox, &encryption_key);
    Ok(base64::engine::general_purpose::STANDARD.encode(&xored))
  }
}

#[cfg(test)]
mod tests {
  use crate::blackbox::Blackbox;
  use crate::fingerprint::Fingerprint;
  use crate::vector::VectorString;
  use chrono::DateTime;
  use std::str::FromStr;

  #[rstest::fixture]
  //noinspection DuplicatedCode, SpellCheckingInspection
  fn blackbox_inst() -> Blackbox {
    Blackbox::new(Fingerprint {
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
    })
  }

  #[rstest::fixture]
  //noinspection SpellCheckingInspection
  fn blackbox_encoded_string() -> String {
    "tra:JVqc1fosb5TG-D2yJJMDaI2_BVy9L6IDep_RAyhanQNk0EOozf9CZ5nLDXniULvgEkRpm94DNWezHIr_d5zOACVXmr_xI2bOQK8cgabYCi9hpMn7LXTjUrkliq_hEVrIK1l-sOIHOXy02QtOf7HWCEtwotQ5p9QpfKHTFobyF0mM8V-EtugNP4Kn2QtxpdwPc6feEUFypQk-dtk-o9oSSXyuE0yDtxt_sxR6rBFJrd0OPnOn2xR3quIWTIG5HH7kFUp6rxRIeN1BptsLMGKUuesuU4W3_m3cQ68UOWub5FK14wg6apLTIGSNsuQnaLb9SY6z5RU9fssPNGapzgAwcb4CJ1mJ2zygBXTiBzlpuxM4aprPBjZmi73tRZm-8CBIuht_5FPBNJ3C9CSS82nSAzNYiroGUqj1Gkx8reUTRHKq0_gqbZLE9EOzGIbNGT5woOU4XY-_8iBSe6DSBClbnsP1J12-IFO160605xd7rOIWet4VS4TqG3636Bl83kR62w1z2D5v1Ak9bdI1m9AJPZ_UBDZmzAE2mv8vkfRavSFZjcHmGEpvoeQJO23RCmvRCGnK-1-PvyVXh7keVojB8li98SqMxfgsYMb8NWyg1gw_dNk-n9QHbKMIacwxZ58CM2PJ_CxewSRdkMMkSXut0gRHeLHjEzhqrd4ORnabzRBCdpvNEDVnmcwDOXDRCTyf1AVozC1gmcktksPzWIvC9Spau_ElVbnxU7joSa_kGlOMvR9UuvAlh7fvIFiIwSJYvO4ni_EhVLrfEUNomt0CNGab_CyR9ylfxSliyC5el80DO3TaP6ABZZX5LWbMLmab0AE5a50CY5n7LpD1LGDB8SNak8cqXL0eT3_kRanaDj-lyvwuU4XI7R9Rh-hNr-UZSnzeEESmBz6iBjZupNkMcdMIOGmg2Qk7cafXCT12r-gZULHnG0t7rOMXfbYXe7IXULIXfbTsTa8VR2ye0PUnao_B8yVWh7frHoK561GF6Rl7rOFGeN5EpQdApQhs0QI4ntU4bM4Ga57XB2qhA2nL_y-Uy_1jx_lewSVcj_BRg-YZfrPYCjxhk9YHOW2by_8yZp3SBDtwodcHPnKXyQwxY5X5MJPI-1_E9CmMwiOI6SGFuyFRgeUdfq_mHlSJ7ydYvCJSs-skV7wihLYbfbPnGEiqDUR4rxF3qOAURnmtEHapzgAyV4nM_TNnl84BOG6m2P0vcpfJ-y1dj8PwIVOAsuo-b6HG-Tpvpcr9Pm-k0gg8dM7zJVd8rvEWSHqqHE-IvzSu4k2G9Cha0wNv4kOoC3qtI1PIPK7TBTdcjtEENmaLvQBu40-74BJVeqzeP6zhUaj9U4TXL332W67vIHeqFYjXL5PnMmW3A1vGE4HbDEWsEFWvI3nPIWrBGX21Bl60BmyxHW69AFq7DGPaDXHGCz6U3ESL4xVe0ydvyPxVrSGR6RmQBlit5Qo8gssSX5fqG4fTKJPgWLv_OLIHTsH1V9AiWbAIf7YGb5_UJGyyAkzGMqf1bq8nde88bbo0nxZkuBtMmRNckLXnGT5ws9gKPIn4cttHsxQ5a7HmFERpm8vzS3yt0gVHbJ7OGoPxZt4DNWXdFUuq4BQ9YpTEBXXlUbYNctQfiPwhU5nOAThmmc_0JlZ-yRFlsv4jVZi97x-L9F_E6RtLkvdaxTRdgrTkJ48BcN1CZ5nfEEN0otIAMF6Os-UVaMkvkAJrkMIIPXCn1Qg-Y5XH7B5hhrjqHEx-st8QQm-h2S1ekLXoKV6UuewtXpPtEkR2m80QfvNfy_AlaQ".to_string()
  }

  #[rstest::fixture]
  //noinspection SpellCheckingInspection
  fn blackbox_encrypted_string() -> String {
    "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".to_string()
  }

  #[rstest::fixture]
  fn gsid() -> String {
    "4fcf4367-1a2e-48b8-9b9a-129fae8a8e5c".to_string()
  }

  #[rstest::fixture]
  fn account_id() -> String {
    "67e55044-10b1-426f-9247-bb680e5fe0c8".to_string()
  }

  #[rstest::rstest]
  fn should_correctly_serialize(blackbox_inst: Blackbox, blackbox_encoded_string: String) {
    let res = serde_plain::to_string(&blackbox_inst);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), blackbox_encoded_string);
  }

  #[rstest::rstest]
  fn should_correctly_deserialize(blackbox_encoded_string: String, blackbox_inst: Blackbox) {
    let res = serde_plain::from_str::<Blackbox>(&blackbox_encoded_string);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), blackbox_inst);
  }

  #[rstest::rstest]
  fn should_correctly_encrypt(
    blackbox_inst: Blackbox,
    gsid: String,
    account_id: String,
    blackbox_encrypted_string: String,
  ) {
    let res = blackbox_inst.encrypt(gsid, account_id);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), blackbox_encrypted_string);
  }

  #[rstest::rstest]
  fn should_correctly_decrypt(
    blackbox_encrypted_string: String,
    gsid: String,
    account_id: String,
    blackbox_inst: Blackbox,
  ) {
    let res = Blackbox::decrypt(blackbox_encrypted_string, gsid, account_id);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), blackbox_inst);
  }
}
