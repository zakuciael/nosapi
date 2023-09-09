use std::fs::File;
use std::io::Read;

use nosapi_data::client::{get_client_version, ClientHashes, ClientVersion};

#[test]
/// Checks if the get_client_version returns correct client version from the specified input
fn returns_correct_client_version() {
  let mut client_x = vec![];
  let mut client_gl = vec![];

  File::open("./tests/fixtures/NostaleClientX.exe")
    .expect("Failed to open NostaleClientX.exe")
    .read_to_end(&mut client_x)
    .expect("Failed to read NostaleClientX.exe");
  File::open("./tests/fixtures/NostaleClient.exe")
    .expect("Failed to open NostaleClient.exe")
    .read_to_end(&mut client_gl)
    .expect("Failed to read NostaleClient.exe");

  let result = get_client_version(&client_x, &client_gl).expect("Failed to get client version");

  assert_eq!(
    result,
    Some(ClientVersion {
      version: "0.9.3.3202".to_owned(),
      hashes: ClientHashes {
        client_x: "c8f3f25403b31d725f83648a338a95c6".to_owned(),
        client_gl: "3b56b242d0b9da85d5c5f294e10e5c03".to_owned()
      }
    })
  )
}

#[test]
/// Checks if the get_client_version correctly handles errors
fn handles_errors_correctly() {
  let client_x = vec![
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  ];

  assert!(get_client_version(&client_x, &vec![]).is_err())
}
