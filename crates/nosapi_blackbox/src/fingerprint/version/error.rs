use crate::fingerprint::version::SUPPORTED_FINGERPRINT_VERSION;

#[derive(thiserror::Error, Debug)]
pub enum FingerprintVersionError {
    #[error(
        "unsupported fingerprint version. received: {0}, expected: {SUPPORTED_FINGERPRINT_VERSION}"
    )]
    UnsupportedVersion(u32),
}
