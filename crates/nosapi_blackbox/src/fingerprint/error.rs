//! Errors related to the `fingerprint` and `request` structs.

/// An error emitted when a method is supplied with an invalid `gsid` value.
#[derive(thiserror::Error, Debug)]
#[error("Invalid gsid")]
pub struct InvalidGsid;
