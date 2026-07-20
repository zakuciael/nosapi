//! Errors returned when parsing fingerprint schema versions.

use crate::fingerprint::version::SUPPORTED_FINGERPRINT_VERSION;

/// Error returned when a fingerprint uses an unsupported schema version.
#[derive(thiserror::Error, Debug)]
pub enum FingerprintVersionError {
    /// The decoded version does not match [`SUPPORTED_FINGERPRINT_VERSION`].
    #[error(
        "unsupported fingerprint version. received: {0}, expected: {SUPPORTED_FINGERPRINT_VERSION}"
    )]
    UnsupportedVersion(u32),
}
