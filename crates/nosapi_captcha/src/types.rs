//! Data types used in the API

use chrono::{DateTime, Utc};
use core::fmt;
use serde::{Deserialize, Serialize};

/// Information about the captcha challenge
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "status")]
#[serde(rename_all = "camelCase")]
pub enum Captcha {
  #[serde(rename = "presented")]
  Unsolved(CaptchaData),
  Solved(CaptchaData),
}

/// Inner struct used in the [`Captcha`] enum.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CaptchaData {
  /// Captcha challenge id.
  #[serde(rename = "id")]
  pub challenge_id: String,
  /// Unix timestamp indicating when the challenge was last updated.
  #[serde(with = "chrono::serde::ts_milliseconds")]
  pub last_updated: DateTime<Utc>,
}

/// A struct containing raw captcha resources
pub struct Resources {
  pub text: bytes::Bytes,
  pub drag_icons: bytes::Bytes,
  pub drop_target: bytes::Bytes,
}

/// Locales supported by the API
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
