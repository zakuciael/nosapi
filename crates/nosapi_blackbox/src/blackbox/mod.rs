//! Implementation of the `blackbox` string.

mod de;
pub mod error;
mod ser;

use self::error::{DecryptBlackboxError, EncryptBlackboxError};
use crate::fingerprint::Fingerprint;
use base64::Engine;
use sha2::Digest;

pub(crate) fn create_encryption_key(gsid: String, account_id: String) -> Vec<u8> {
    let key = format!("{}-{}", gsid, account_id);
    let hash = sha2::Sha512::digest(&key);

    hex::encode(hash).into()
}

fn xor(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
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
    use crate::blackbox::create_encryption_key;
    use crate::{Blackbox, Fingerprint, VectorString};
    use chrono::DateTime;
    use std::str::FromStr;

    #[rstest::fixture]
    //noinspection DuplicatedCode, SpellCheckingInspection
    fn blackbox_inst() -> Blackbox {
        Blackbox(Fingerprint {
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
        })
    }

    #[rstest::fixture]
    //noinspection SpellCheckingInspection
    fn blackbox_encoded_string() -> String {
        "tra:JVqczf8kVpm-8CJn3E69LZK36S9x5kqrG4DzZ4y-8BVHiq_hE2rTQaUUi_4jVYes3iFGeKrtVcc2owgtX5G26CtQgrT7atlArBE2aJjhT7LgBTdpjsADO2CS1QY4XY_S9ylbwC5bsAMoWp0CcJXH-R5Qk7jqHFC26x6B5UZ3rxJEpgdomPsrXpPHKYvAJl_A81i7HYK3HICx40Sm2j522ww9nwA4b6IFN53O_zBml8n5K43G-i9UhrjdD1J3qdsikQBn0zhdj78IdtkHLF6h4jB3wwgtX4-3-EWJruAQYsMnjPtpjsDwQpq_8SFaisHxFkh40CRJe6vTJWaqACVXh84UbJ3P_zBZfrDgJI3_ZMc7brLjFDlrmxGE4xh3p8z-Lp4RcKUENF2CtOYLPYCl1wk8b6beDkV32w1u0AM2a9AIPG6nDXCk1gc3aJjILZLI-y9hkvRYj8L2WpHBJ1qKwydbwPgrYpPKLF3BJVeJ7E5-o9UHLF6hxvgqjscojsUmh7gcTHziFER22xNFfq8Veq7nSYK16R2DufIpXZPJ_DGW-1yRxClgxSaJ7iRcv_AghrnpG37hGk2A4QY4ao_BBDZrodH2KGuc0AQ0WYvO8yVXvB1PsuUegbYbft8TS4PlSYDlSnrbDHCnCm-lBzxyotI2l8wyZZv8M5bOL5DIAWPHLI6-9S1jnMwyl88IPW3SCC1fkbboK1CCtOgdU4nB-F2R9iZWiMHyJFm87E6FuB2B5xpThOpLe60SR6nhGkqA5UuA4kV42Txu0AM3nP5jnM8xksL2WYq97U5zpdf8LnGWyPpbvu4nirwegrfpTITmFkmu50x-r-VIqeESRny15kes4BN0qAs-d63iE0mq4RJJqtoSdabaEXXaEkqC5hlMhbogRXepzgBDaJrM_jWbzQE0lfYmh-oiVrfsI1m_8iJSi8PzKYy_IVKFu-wggbUagOISQ6TdPnKk2Ao6ap7PM5TEJl2P9FeL7ieJv-QWSG2f4hNFeafXCz5ypt0RR3yv5hlSd6nsEUN12z2jBT5yp99Dc6MJPaIFN50DZMb3KpDD-Vq871CCufFSheYdgbXsUovC9ilbvvcrYpsBN2ug0QptoNM3m8wBOV6QwucZXI298SJajr73KFh9r_IXSXut3Q9FcqLZBjZqvu4nTH_A9ChNgMHzKFaOw_VPdKbY_S9yl8n7ZdkNRHTpWMA3qiNTwy2R9WbbSrgvZNk_pgc_ZJbI7R9ilMz8IVOWu-0fUICl1wkuYKPI-iyGzhxesAc_rA584FO46TKm_WbABE-7AG27D2Gt-U-I0DSHwCd_sCSL3TN15kmcEGewHF7BEGa77kaa3DSa3Q5htegqjdYGVLcZSbAYa74uYcY0lwNXrx-B5BRd1CJlt-k8g7cvkwBGlewwoPlajPxSo9VNovM7bKLvWb3_VJflTK4XfrcFbq8nde9W0B1xxvpHsQo6iPI7stcJO2CS1fosXqsalP1p1TZbjdMINmaLve0VbNVDpxaNACVXh9UpToCw4RE_b5THCS5gkOc2jcP3IEV3p-hYyDSZ8FW3AmvfBDZ8seQbSXyy1wk5Yaz0SJXhBjh7oNICbtdCp8z-LnXaPagXQGWXxwpy5FPAJUp8wvkrWYm36iBSiLbnGUpvodEkhetMvidMfsT5LGORxPofUYOo2h1CdKbYCDpwnc0EMWGV6RlSd6rrH1N4q-weU63SBDZbjdA-sx-LsOIlXI_G-DBVis4".to_string()
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
    fn should_correctly_encode(blackbox_inst: Blackbox) {
        let res = blackbox_inst.encode();

        assert!(res.is_ok());
        insta::assert_snapshot!(res.unwrap());
    }

    #[rstest::rstest]
    fn should_correctly_encrypt(blackbox_inst: Blackbox, gsid: String, account_id: String) {
        let res = blackbox_inst.encrypt(gsid, account_id);

        assert!(res.is_ok());
        insta::assert_snapshot!(res.unwrap());
    }

    #[rstest::rstest]
    fn should_correctly_decode(blackbox_encoded_string: String, blackbox_inst: Blackbox) {
        let res = Blackbox::decode(blackbox_encoded_string);

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), blackbox_inst);
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

    #[rstest::rstest]
    fn should_generate_correct_encryption_key(gsid: String, account_id: String) {
        let key = create_encryption_key(gsid, account_id);

        assert_eq!(key, "e068ac36cc43f96bfba7f8923d71eb6bee3e5e2a3ae6da000758786ce9d96a047367b4148fc871a11a76060c34820d31e35368e8136bc2d802bde0488c2d185e".as_bytes());
    }
}
