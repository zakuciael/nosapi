mod blackbox;
mod fingerprint;
pub(crate) mod support;
pub mod vector;

pub use self::blackbox::Blackbox;
pub use self::fingerprint::{Fingerprint, Request};
pub use self::vector::VectorString;

#[cfg(feature = "builders")]
pub use self::fingerprint::{FingerprintBuilder, RequestBuilder};
#[cfg(feature = "builders")]
pub use self::vector::VectorStringBuilder;

pub mod error {
  pub use super::blackbox::error::*;
  pub use super::vector::error::*;
}

pub mod prelude {
  pub use super::error::*;
  pub use super::{Blackbox, Fingerprint, Request, VectorString};

  #[cfg(feature = "builders")]
  pub use super::VectorStringBuilder;
  #[cfg(feature = "builders")]
  pub use super::{FingerprintBuilder, RequestBuilder};
}
