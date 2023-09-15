use std::io::{Read, Seek};

use crate::nos::error::NOSFileHeaderError;

static DATA_FILE_HEADER: [&str; 3] = ["NT Data", "32GBS V1.0", "ITEMS V1.0"];
static CCINF_FILE_HEADER: &str = "CCINF V1.20";

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum NOSFileType {
  Data(Option<NOSDataType>),
  Text,
  CCInf,
}

impl NOSFileType {
  pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(bytes: &T) -> Result<Self, NOSFileHeaderError> {
    let bytes = bytes.as_ref();

    if bytes.len() != 0x0B {
      return Err(NOSFileHeaderError(format!(
        "File header has invalid size, expected 0x0B got {:#04X}",
        bytes.as_ref().len()
      )));
    }

    let header = String::from_utf8_lossy(bytes);
    Ok(match header {
      _ if &header[0..7] == DATA_FILE_HEADER[0]
        || &header[0..10] == DATA_FILE_HEADER[1]
        || &header[0..10] == DATA_FILE_HEADER[2] =>
      {
        NOSFileType::Data(NOSDataType::from_header(&header)?)
      }
      _ if &header[0..11] == CCINF_FILE_HEADER => NOSFileType::CCInf,
      _ => NOSFileType::Text,
    })
  }

  pub fn from_reader<T: Read + Seek>(reader: &mut T) -> Result<Self, NOSFileHeaderError> {
    let mut buf = [0u8; 0x0B];
    reader
      .read_exact(&mut buf)
      .map_err(|_| NOSFileHeaderError("Failed to read the header".to_string()))?;

    Self::from_bytes(&buf)
  }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(usize)]
pub enum NOSDataType {
  PCHPKG = 1,
  NStuData = 2,
  NStkData = 3,
  NStcData = 5,
  NStgData = 6,
  NStpData = 7,
  NStsData = 9,
  NStgeData = 10,
  NStpeData = 11,
  NStpuData = 12,
  NSpcData = 13,
  NSppData = 14,
  NSpmData = 15,
  NSmcData = 16,
  NSmpData = 17,
  NSedData = 20,
  NSemData = 21,
  NSesData = 22,
  NSeffData = 23,
  NSipData = 24,
  NSgrdData = 26,
  NS4BbData = 101,
  NSipData2006 = 103,
}

impl NOSDataType {
  pub fn from_header(header: &str) -> Result<Option<Self>, NOSFileHeaderError> {
    match header {
      _ if &header[0..7] == DATA_FILE_HEADER[0] => {
        let raw: usize = header[8..10].parse::<usize>().map_err(|_| {
          NOSFileHeaderError("File header doesn't contain an valid type number".to_string())
        })?;

        Ok(Self::from_raw(raw))
      }
      _ if &header[0..10] == DATA_FILE_HEADER[1] => Ok(Some(Self::NS4BbData)),
      _ if &header[0..10] == DATA_FILE_HEADER[2] => Ok(Some(Self::NSipData2006)),
      _ => Ok(None),
    }
  }

  pub fn from_raw(raw: usize) -> Option<Self> {
    match raw {
      _ if raw == Self::PCHPKG as usize => Some(Self::PCHPKG),
      _ if raw == Self::NStuData as usize => Some(Self::NStuData),
      _ if raw == Self::NStkData as usize => Some(Self::NStkData),
      _ if raw == Self::NStcData as usize => Some(Self::NStcData),
      _ if raw == Self::NStgData as usize => Some(Self::NStgData),
      _ if raw == Self::NStpData as usize => Some(Self::NStpData),
      _ if raw == Self::NStsData as usize => Some(Self::NStsData),
      _ if raw == Self::NStgeData as usize => Some(Self::NStgeData),
      _ if raw == Self::NStpeData as usize => Some(Self::NStpeData),
      _ if raw == Self::NStpuData as usize => Some(Self::NStpuData),
      _ if raw == Self::NSpcData as usize => Some(Self::NSpcData),
      _ if raw == Self::NSppData as usize => Some(Self::NSppData),
      _ if raw == Self::NSpmData as usize => Some(Self::NSpmData),
      _ if raw == Self::NSmcData as usize => Some(Self::NSmcData),
      _ if raw == Self::NSmpData as usize => Some(Self::NSmpData),
      _ if raw == Self::NSedData as usize => Some(Self::NSedData),
      _ if raw == Self::NSemData as usize => Some(Self::NSemData),
      _ if raw == Self::NSesData as usize => Some(Self::NSesData),
      _ if raw == Self::NSeffData as usize => Some(Self::NSeffData),
      _ if raw == Self::NSipData as usize => Some(Self::NSipData),
      _ if raw == Self::NSgrdData as usize => Some(Self::NSgrdData),
      _ => None,
    }
  }
}
