//! Types returned by the Gameforge captcha API.

use chrono::{DateTime, Utc};
use core::fmt;
use serde::{Deserialize, Serialize};

/// Current state of a captcha challenge.
///
/// A challenge can be returned as already solved or as still presented to the
/// user. Both variants carry the same [`CaptchaData`] payload.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "status")]
#[serde(rename_all = "camelCase")]
pub enum Captcha {
    /// The challenge still needs an answer.
    #[serde(rename = "presented")]
    Unsolved(CaptchaData),
    /// The challenge has already been solved.
    Solved(CaptchaData),
}

/// Metadata shared by solved and unsolved captcha states.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CaptchaData {
    /// Captcha challenge id used in subsequent resource and answer requests.
    #[serde(rename = "id")]
    pub challenge_id: String,
    /// Timestamp indicating when the displayed challenge was last updated.
    ///
    /// Gameforge expects this timestamp, in milliseconds, as the query string on
    /// resource URLs. Use [`crate::Client::resources`] to construct those URLs
    /// for you.
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub last_updated: DateTime<Utc>,
}

/// Raw resources needed to render a captcha challenge.
///
/// The bytes are returned exactly as served by the API. The `text` resource is
/// the localized instruction payload, while `drag_icons` and `drop_target` are
/// image payloads suitable for writing to files or serving directly to a UI.
pub struct Resources {
    /// Localized instruction text displayed above the image-drop challenge.
    pub text: bytes::Bytes,
    /// Sprite sheet containing the selectable drag icons.
    pub drag_icons: bytes::Bytes,
    /// Image for the drop target.
    pub drop_target: bytes::Bytes,
}

/// Locale used when fetching challenge text and images.
///
/// Formatting a value with [`ToString::to_string`] returns the locale code
/// expected by the Gameforge endpoint, for example `Locale::EnglishUS` becomes
/// `en-US`.
#[non_exhaustive]
pub enum Locale {
    /// Bosnian (Bosnia and Herzegovina)
    Bosnian,
    /// Czech (Czechia)
    Czech,
    /// Danish (Denmark)
    Danish,
    /// German (Germany)
    German,
    /// Estonian (Estonia)
    Estonian,
    /// English (United Kingdom)
    EnglishUK,
    /// English (United States of America)
    EnglishUS,
    /// Spanish (Spain)
    Spanish,
    /// Spanish (Argentina)
    SpanishAR,
    /// Spanish (Colombia)
    SpanishCO,
    /// Spanish (Mexico)
    SpanishMX,
    /// Spanish (Peru)
    SpanishPE,
    /// French (France)
    French,
    /// Croatian (Croatia)
    Croatian,
    /// Italian (Italy)
    Italian,
    /// Latvian (Latvia)
    Latvian,
    /// Lithuanian (Lithuania)
    Lithuanian,
    /// Hungarian (Hungary)
    Hungarian,
    /// Dutch (Netherlands)
    Dutch,
    /// Norwegian Bokmål (Norway)
    Norwegian,
    /// Polish (Poland)
    Polish,
    /// Portuguese (Brazil)
    PortugueseBR,
    /// Portuguese (Portugal)
    PortuguesePT,
    /// Romanian (Romania)
    Romanian,
    /// Slovene (Slovenia)
    Slovene,
    /// Slovak (Slovakia)
    Slovak,
    /// Finnish (Finland)
    Finnish,
    /// Swedish (Sweden)
    Swedish,
    /// Turkish (Turkey)
    Turkish,
    /// Modern Greek (Greece)
    Greek,
    /// Bulgarian (Bulgaria)
    Bulgarian,
    /// Russian (Russia)
    Russian,
    /// Serbian (Serbia)
    Serbian,
    /// Ukrainian (Ukraine)
    Ukrainian,
    /// Modern Hebrew (Israel)
    Hebrew,
    /// Arabic (United Arab Emirates)
    Arabic,
    /// Japanese (Japan)
    Japanese,
    /// Taiwan (Taiwan)
    Chinese,
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Bosnian => "bs-BA",
            Self::Czech => "cs-CZ",
            Self::Danish => "da-DK",
            Self::German => "de-DE",
            Self::Estonian => "et-EE",
            Self::EnglishUK => "en-GB",
            Self::EnglishUS => "en-US",
            Self::Spanish => "es-ES",
            Self::SpanishAR => "es-AR",
            Self::SpanishCO => "es-CO",
            Self::SpanishMX => "es-MX",
            Self::SpanishPE => "es-PE",
            Self::French => "fr-FR",
            Self::Croatian => "hr-HR",
            Self::Italian => "it-IT",
            Self::Latvian => "lv-LV",
            Self::Lithuanian => "lt-LT",
            Self::Hungarian => "hu-HU",
            Self::Dutch => "nl-NL",
            Self::Norwegian => "nb-NO",
            Self::Polish => "pl-PL",
            Self::PortugueseBR => "pt-BR",
            Self::PortuguesePT => "pt-PT",
            Self::Romanian => "ro-RO",
            Self::Slovene => "sl-SI",
            Self::Slovak => "sk-SK",
            Self::Finnish => "fi-FI",
            Self::Swedish => "sv-SE",
            Self::Turkish => "tr-TR",
            Self::Greek => "el-GR",
            Self::Bulgarian => "bg-BG",
            Self::Russian => "ru-RU",
            Self::Serbian => "sr-RS",
            Self::Ukrainian => "uk-UA",
            Self::Hebrew => "he-IL",
            Self::Arabic => "ar-AE",
            Self::Japanese => "ja-JP",
            Self::Chinese => "zh-TW",
        };

        write!(f, "{value}")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CaptchaAnswer {
    pub(crate) answer: u8,
}
