//! Contains functions related to the game's client EXEs

use pelite::pe32::{Pe, PeFile};

use crate::client::error::ClientVersionError;

pub mod error;

/// Stores MD5 hashes of the game's client EXEs
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ClientHashes {
  /// MD5 hash of `NostaleClientX.exe`
  pub client_x: String,
  /// MD5 hash of `NostaleClient.exe`
  pub client_gl: String,
}

/// Represents the game's client version and hashes of its EXEs
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClientVersion {
  /// Game's client version
  pub version: String,
  /// MD5 hashes of game's client EXEs
  pub hashes: ClientHashes,
}

/// Resolves a client version and computes MD5 hashes of game's client EXEs
///
/// # Examples
/// ```
/// use nosapi_data::client::get_client_version;
/// use std::error::Error;
/// use std::fs::File;
/// use std::io::Read;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///   let mut client_x = vec![];
///   let mut client_gl = vec![];
///
///   File::open("./tests/fixtures/NostaleClientX.exe")?
///     .read_to_end(&mut client_x)?;
///   File::open("./tests/fixtures/NostaleClient.exe")?
///     .read_to_end(&mut client_gl)?;
///
///   if let Some(result) = get_client_version(&client_x, &client_gl)? {
///     println!("{:?}", result.version); // 0.9.3.3202
///   }
///
///   Ok(())
/// }
/// ```
///
/// # Errors
/// - [`InvalidFile`](ClientVersionError::InvalidFile) - Specified file is not a valid [PE32 file](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format)
/// - [`NoResources`](ClientVersionError::NoResources) - Specified file doesn't contain the [`resources`](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#the-rsrc-section) section
/// - [`UnableToFindVersionInfo`](ClientVersionError::UnableToFindVersionInfo) - The version information was not found in the EXE
pub fn get_client_version<X: AsRef<[u8]> + ?Sized, GL: AsRef<[u8]> + ?Sized>(
  client_x: &X,
  client_gl: &GL,
) -> Result<Option<ClientVersion>, ClientVersionError> {
  let version = PeFile::from_bytes(client_x)
    .map_err(ClientVersionError::InvalidFile)?
    .resources()
    .map_err(|_| ClientVersionError::NoResources)?
    .version_info()
    .map_err(|_| ClientVersionError::UnableToFindVersionInfo)?
    .fixed()
    .map(|info| info.dwProductVersion);

  Ok(version.map(|version| ClientVersion {
    version: version.to_string(),
    hashes: ClientHashes {
      client_x: format!("{:x}", md5::compute(client_x)),
      client_gl: format!("{:x}", md5::compute(client_gl)),
    },
  }))
}
