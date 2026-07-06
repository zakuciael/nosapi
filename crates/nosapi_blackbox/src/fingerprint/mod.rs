//! Implementation of the `fingerprint` struct found in the `blackbox` string.

use rand::RngExt;

pub mod error;
pub mod version;

use crate::fingerprint::version::FingerprintVersion;
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
        let features = rng_generator().random_range(1..9999);
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
    pub version: FingerprintVersion,
    pub timezone: String,
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
    pub request: Option<Request>,
    pub browser_env_mask: u32,
}

#[cfg(test)]
mod tests {
    use crate::fingerprint::Request;
    use crate::{Fingerprint, VectorString};
    use chrono::DateTime;
    use serde::Serialize;
    use serde::Serializer;
    use serde_tuple_explicit::{DeserializeTuple, SerializeTuple};
    use std::str::FromStr;

    #[rstest::fixture]
    //noinspection DuplicatedCode, SpellCheckingInspection
    fn fingerprint_inst() -> Fingerprint {
        Fingerprint {
            version: 12.try_into().unwrap(),
            timezone: "Europe/Budapest".to_string(),
            os_name: "Windows".to_string(),
            browser_name: "Chrome".to_string(),
            vendor: "Google Inc.".to_string(),
            memory: 8,
            concurrency: 12,
            languages: "en-US,en".to_string(),
            plugins_hash: "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945".to_string(),
            gpu: "Google Inc.,ANGLE (AMD Radeon RX 9070 XT (RADV GFX1201) Direct3D11 vs_5_0 ps_5_0)".to_string(),
            fonts_hash: "3378072d2ab335e8429fc4210100ee63421bd734d70f309d4e83717b1dd22cb0".to_string(),
            audio_context_hash: "d9af7aa1d00f202e8291fe49b9344f69746635eea53e7eace68c10f302cc933a".to_string(),
            width: 2560,
            height: 1440,
            video_codecs_hash: "ea2c39c5eca488bd7ee0a1d7ce6b5600da5f36a7c8aa89bdeb078690fe8950e6".to_string(),
            audio_codecs_hash: "456687e4e0029125c0b73edf391fa02e5b8906ef5bc3ac2b34ebe93ba04c130a".to_string(),
            media_devices_hash: "ac09c2bd52c8b03e9e216ca814691ae43a4c396516a717a08c147de888d3395f".to_string(),
            permissions_hash: "27f243aa0ac84a576f3009806c3b13614a4efb01a9a42420041da0b72ec4c9b6".to_string(),
            audio_fingerprint: 124.0434474653739,
            webgl_fingerprint: "fbfb9458d00f4ec2ffab13f36ab3a278a3a7d47f97432c9479f64519c33dd158".to_string(),
            canvas_fingerprint: 1041840910,
            creation: FromStr::from_str("2026-07-04T09:44:25.852Z").unwrap(),
            game: "jt470uohwsy0pjddquonw5ufga8".to_string(),
            delta: 280,
            os_version: Some("10".to_string()),
            vector: VectorString::new(
                "dsAEo&nwl{R-Z&C*Q&54K-_Gu/`_[`EPjq+V\"P\\9U7]0W|-ROp\\#C\\oH!I*wzw%Mz[sB04$vHn1vaNX:XkjUClT@}z27AP#`n(=6".to_string(),
                DateTime::from_timestamp_millis(1783158264620).unwrap()
            ),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3626.121 Safari/537.36".to_string(),
            server_time: FromStr::from_str("2026-07-04T09:44:25Z").unwrap(),
            request: None,
            browser_env_mask: 73728,
        }
    }

    #[rstest::fixture]
    fn fingerprint_json() -> String {
        r#"{
            "version": 12,
            "timezone": "Europe/Budapest",
            "os_name": "Windows",
            "browser_name": "Chrome",
            "vendor": "Google Inc.",
            "memory": 8,
            "concurrency": 12,
            "languages": "en-US,en",
            "plugins_hash": "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
            "gpu": "Google Inc.,ANGLE (AMD Radeon RX 9070 XT (RADV GFX1201) Direct3D11 vs_5_0 ps_5_0)",
            "fonts_hash": "3378072d2ab335e8429fc4210100ee63421bd734d70f309d4e83717b1dd22cb0",
            "audio_context_hash": "d9af7aa1d00f202e8291fe49b9344f69746635eea53e7eace68c10f302cc933a",
            "width": 2560,
            "height": 1440,
            "video_codecs_hash": "ea2c39c5eca488bd7ee0a1d7ce6b5600da5f36a7c8aa89bdeb078690fe8950e6",
            "audio_codecs_hash": "456687e4e0029125c0b73edf391fa02e5b8906ef5bc3ac2b34ebe93ba04c130a",
            "media_devices_hash": "ac09c2bd52c8b03e9e216ca814691ae43a4c396516a717a08c147de888d3395f",
            "permissions_hash": "27f243aa0ac84a576f3009806c3b13614a4efb01a9a42420041da0b72ec4c9b6",
            "audio_fingerprint": 124.0434474653739,
            "webgl_fingerprint": "fbfb9458d00f4ec2ffab13f36ab3a278a3a7d47f97432c9479f64519c33dd158",
            "canvas_fingerprint": 1041840910,
            "creation": "2026-07-04T09:44:25.852Z",
            "game": "jt470uohwsy0pjddquonw5ufga8",
            "delta": 280,
            "os_version": "10",
            "vector": "ZHNBRW8mbndse1ItWiZDKlEmNTRLLV9HdS9gX1tgRVBqcStWIlBcOVU3XTBXfC1ST3BcI0Ncb0ghSSp3enclTXpbc0IwNCR2SG4xdmFOWDpYa2pVQ2xUQH16MjdBUCNgbig9NiAxNzgzMTU4MjY0NjIw",
            "user_agent": "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3626.121 Safari/537.36",
            "server_time": "2026-07-04T09:44:25Z",
            "request": null,
            "browser_env_mask": 73728
        }"#.to_string()
    }

    #[rstest::fixture]
    fn fingerprint_array() -> String {
        r#"[
            12,
            "Europe/Budapest",
            "Windows",
            "Chrome",
            "Google Inc.",
            8,
            12,
            "en-US,en",
            "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
            "Google Inc.,ANGLE (AMD Radeon RX 9070 XT (RADV GFX1201) Direct3D11 vs_5_0 ps_5_0)",
            "3378072d2ab335e8429fc4210100ee63421bd734d70f309d4e83717b1dd22cb0",
            "d9af7aa1d00f202e8291fe49b9344f69746635eea53e7eace68c10f302cc933a",
            2560,
            1440,
            "ea2c39c5eca488bd7ee0a1d7ce6b5600da5f36a7c8aa89bdeb078690fe8950e6",
            "456687e4e0029125c0b73edf391fa02e5b8906ef5bc3ac2b34ebe93ba04c130a",
            "ac09c2bd52c8b03e9e216ca814691ae43a4c396516a717a08c147de888d3395f",
            "27f243aa0ac84a576f3009806c3b13614a4efb01a9a42420041da0b72ec4c9b6",
            124.0434474653739,
            "fbfb9458d00f4ec2ffab13f36ab3a278a3a7d47f97432c9479f64519c33dd158",
            1041840910,
            "2026-07-04T09:44:25.852Z",
            "jt470uohwsy0pjddquonw5ufga8",
            280,
            "10",
            "ZHNBRW8mbndse1ItWiZDKlEmNTRLLV9HdS9gX1tgRVBqcStWIlBcOVU3XTBXfC1ST3BcI0Ncb0ghSSp3enclTXpbc0IwNCR2SG4xdmFOWDpYa2pVQ2xUQH16MjdBUCNgbig9NiAxNzgzMTU4MjY0NjIw",
            "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3626.121 Safari/537.36",
            "2026-07-04T09:44:25.000Z",
            null,
            73728
        ]"#.to_string()
    }

    #[rstest::fixture]
    fn request_inst(installation_id: String) -> Request {
        Request {
            features: vec![7310],
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
    fn should_correctly_serialize_to_json(fingerprint_inst: Fingerprint) {
        insta::assert_json_snapshot!(fingerprint_inst);
    }

    #[rstest::rstest]
    fn should_correctly_serialize_to_array(fingerprint_inst: Fingerprint) {
        struct FingerprintWrapper(Fingerprint);

        impl Serialize for FingerprintWrapper {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                self.0.serialize_tuple(serializer)
            }
        }

        insta::assert_json_snapshot!(FingerprintWrapper(fingerprint_inst));
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
