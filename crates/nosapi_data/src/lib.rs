//! Library for working with [NosTale's](https://gameforge.com/pl-PL/play/nostale) client files.
//!
//! # Create features
//! This library uses a set of [feature flags](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section) to reduce the amount of compiled code.
//! It is possible to just enable certain features over other.
//! By default,
//! this crate does not enable any features but allows one to enable a subset for their use case.
//! Below is a list of the available feature flags.
//!
//! | Feature Name | Description |
//! |-|-|
//! | `serde` | Adds support for serialization using the [`serde`](https://crates.io/crates/serde) crate |
//! | `json_schema` | Adds support for generating JSON schemas using the [`schemars`](https://crates.io/crates/schemars) crate |
#![doc(
  html_logo_url = "https://raw.githubusercontent.com/zakuciael/nosapi/main/assets/logo.png",
  html_favicon_url = "https://raw.githubusercontent.com/zakuciael/nosapi/main/assets/logo.png"
)]

pub mod client;
pub mod nos;
pub mod prelude;
pub mod traits;
