//! Utilities for decoding, editing, encoding, encrypting, and decrypting
//! Gameforge `blackbox` strings.
//!
//! A `blackbox` value contains a browser fingerprint represented by
//! [`Fingerprint`]. This crate exposes the transport encoding used by
//! Gameforge, the encrypted form used with a `gsid` and account id, and helpers
//! for the nested [`VectorString`] value.
//!
//! [`Blackbox`] behaves like a [`Fingerprint`] for normal field access: it
//! implements [`std::ops::Deref`] and [`std::ops::DerefMut`], so you can read
//! and edit fingerprint fields directly on the decoded value. It also
//! implements [`AsRef<Fingerprint>`], [`AsMut<Fingerprint>`], and
//! [`From<Fingerprint>`] for APIs that prefer explicit conversions.
//!
//! # Which method should I use?
//!
//! - Use [`Blackbox::decode`] and [`Blackbox::encode`] for transport strings
//!   that look like `tra:...`.
//! - Use [`Blackbox::decrypt`] and [`Blackbox::encrypt`] for encrypted strings.
//!   These methods require the same `gsid` and account id used by the session.
//! - Use [`VectorString::update`] before re-encoding when you intentionally
//!   modify a fingerprint and need to refresh its modification timestamp.
//!
//! # Decode and inspect a fingerprint
//!
//! ```no_run
//! use nosapi_blackbox::Blackbox;
//!
//! fn inspect(encoded_blackbox: &str) -> Result<(), Box<dyn std::error::Error>> {
//!     let blackbox = Blackbox::decode(encoded_blackbox.to_owned())?;
//!
//!     println!("browser: {}", blackbox.browser_name);
//!     println!("user agent: {}", blackbox.user_agent);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Modify and encode
//!
//! ```no_run
//! use nosapi_blackbox::Blackbox;
//!
//! fn change_locale(encoded_blackbox: &str) -> Result<String, Box<dyn std::error::Error>> {
//!     let mut blackbox = Blackbox::decode(encoded_blackbox.to_owned())?;
//!
//!     blackbox.languages = "en-US,en".to_owned();
//!     blackbox.vector.update();
//!
//!     Ok(blackbox.encode()?)
//! }
//! ```
//!
//! # Decrypt and encrypt
//!
//! ```no_run
//! use nosapi_blackbox::Blackbox;
//!
//! fn round_trip(
//!     encrypted_blackbox: &str,
//!     gsid: &str,
//!     account_id: &str,
//! ) -> Result<String, Box<dyn std::error::Error>> {
//!     let blackbox = Blackbox::decrypt(
//!         encrypted_blackbox.to_owned(),
//!         gsid.to_owned(),
//!         account_id.to_owned(),
//!     )?;
//!
//!     Ok(blackbox.encrypt(gsid.to_owned(), account_id.to_owned())?)
//! }
//! ```

pub mod blackbox;
pub(crate) mod constants;
pub mod fingerprint;
#[cfg(test)]
pub(crate) mod mock;
pub(crate) mod utils;
pub mod vector;

pub use blackbox::Blackbox;
pub use fingerprint::Fingerprint;
pub use vector::VectorString;
