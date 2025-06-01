//! A library for working with Gameforge's blackbox string.
//!
//! See the [`Blackbox`] documentation page for examples.

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
