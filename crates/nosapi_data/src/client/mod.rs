pub mod error;

use crate::client::error::{Error, Result};
use pelite::pe32::{Pe, PeFile};

#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ClientHashes {
  pub client_x: String,
  pub client_gl: String,
}

#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClientVersion {
  pub version: String,
  pub hashes: ClientHashes,
}

pub fn get_client_version<X: AsRef<[u8]> + ?Sized, GL: AsRef<[u8]> + ?Sized>(
  client_x: &X,
  client_gl: &GL,
) -> Result<Option<ClientVersion>> {
  let version = PeFile::from_bytes(client_x)
    .map_err(Error::InvalidFile)?
    .resources()
    .map_err(|_| Error::NoResources)?
    .version_info()
    .map_err(|_| Error::UnableToFindVersionInfo)?
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
