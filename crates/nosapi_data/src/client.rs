use pelite::pe32::{Pe, PeFile};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientVersionError {
  #[error("Invalid file provided, expected a valid pe32 file")]
  InvalidFile(#[from] pelite::Error),
  #[error("No resources section found in the executable file")]
  NoResources,
  #[error("Unable to find version info")]
  UnableToFindVersionInfo,
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ClientHashes {
  pub client_x: String,
  pub client_gl: String,
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClientVersion {
  pub version: String,
  pub hashes: ClientHashes,
}

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