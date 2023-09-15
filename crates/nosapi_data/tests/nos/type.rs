use nosapi_data::nos::error::NOSFileHeaderError;
use nosapi_data::nos::{NOSDataType, NOSFileType};
use nosapi_data::prelude::*;

#[test]
fn handles_invalid_header_size() {
  assert!(NOSFileType::from_bytes(&[0u8; 0xA]).is_err_and(|e| matches!(e, NOSFileHeaderError(_))))
}

#[test]
fn resolve_data_file_type() {
  let headers: [[u8; 0x0B]; 3] = [
    [
      0x4E, 0x54, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20, 0x32, 0x34, 0x00,
    ],
    [
      0x33, 0x32, 0x47, 0x42, 0x53, 0x20, 0x56, 0x31, 0x2e, 0x30, 0x00,
    ],
    [
      0x49, 0x54, 0x45, 0x4d, 0x53, 0x20, 0x56, 0x31, 0x2e, 0x30, 0x00,
    ],
  ];

  let test = |header: &[u8]| {
    assert!(matches!(
      NOSFileType::from_bytes(&header).unwrap(),
      NOSFileType::Data(_)
    ))
  };

  test(&headers[0]);
  test(&headers[1]);
  test(&headers[2]);
}

#[test]
fn resolve_ccinf_file_type() {
  let header: [u8; 0x0B] = [
    0x43, 0x43, 0x49, 0x4E, 0x46, 0x20, 0x56, 0x31, 0x2E, 0x32, 0x30,
  ];

  assert_eq!(
    NOSFileType::from_bytes(&header).unwrap(),
    NOSFileType::CCInf
  )
}

#[test]
fn resolve_text_file_type() {
  let header: [u8; 0x0B] = [
    0x26, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00,
  ];

  assert_eq!(NOSFileType::from_bytes(&header).unwrap(), NOSFileType::Text)
}

#[test]
fn resolve_data_type() {
  let data_types = [
    NOSDataType::PCHPKG,
    NOSDataType::NStuData,
    NOSDataType::NStkData,
    NOSDataType::NStcData,
    NOSDataType::NStgData,
    NOSDataType::NStpData,
    NOSDataType::NStsData,
    NOSDataType::NStgeData,
    NOSDataType::NStpeData,
    NOSDataType::NStpuData,
    NOSDataType::NSpcData,
    NOSDataType::NSppData,
    NOSDataType::NSpmData,
    NOSDataType::NSmcData,
    NOSDataType::NSmpData,
    NOSDataType::NSedData,
    NOSDataType::NSemData,
    NOSDataType::NSesData,
    NOSDataType::NSeffData,
    NOSDataType::NSipData,
    NOSDataType::NSgrdData,
  ];

  for data_type in data_types {
    assert_eq!(
      NOSDataType::from_header(format!("NT Data {:02}", data_type as usize).as_str()).unwrap(),
      Some(data_type)
    )
  }

  let headers = ["32GBS V1.0", "ITEMS V1.0"];
  assert_eq!(
    NOSDataType::from_header(headers[0]).unwrap(),
    Some(NOSDataType::NS4BbData)
  );
  assert_eq!(
    NOSDataType::from_header(headers[1]).unwrap(),
    Some(NOSDataType::NSipData2006)
  );
}
