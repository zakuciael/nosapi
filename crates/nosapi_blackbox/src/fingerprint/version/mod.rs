pub mod error;

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Deref;

/// The currently supported fingerprint schema version.
pub const SUPPORTED_FINGERPRINT_VERSION: u32 = 12;

/// Version of the fingerprint schema.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[serde(transparent)]
pub struct FingerprintVersion(u32);

impl FingerprintVersion {
    /// Creates a fingerprint version from its numeric representation.
    pub fn new(version: u32) -> Result<Self, error::FingerprintVersionError> {
        if version != SUPPORTED_FINGERPRINT_VERSION {
            return Err(error::FingerprintVersionError::UnsupportedVersion(version));
        }

        Ok(Self(version))
    }

    /// Returns the numeric representation of this fingerprint version.
    pub const fn value(self) -> u32 {
        self.0
    }
}

impl Display for FingerprintVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u32> for FingerprintVersion {
    type Error = error::FingerprintVersionError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Deref for FingerprintVersion {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
