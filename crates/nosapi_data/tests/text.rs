use lazy_static::lazy_static;
use nosapi_data::archive::{
  text::{Archive, File},
  ArchiveReader,
};
use rstest::rstest;
use std::path::{Path, PathBuf};

lazy_static! {
  static ref EXPORTS_PATH: PathBuf = PathBuf::from("tests/fixtures/exports/text/");
}

fn compare_files(parsed_file: &File, archive_path: &Path) {
  let export_file_path = (*EXPORTS_PATH)
    .join(archive_path.file_stem().unwrap())
    .join(parsed_file.name());
  assert!(
    export_file_path.exists(),
    "Export file not found: {:?}",
    &export_file_path
  );

  let export_file = std::fs::read(&export_file_path)
    .unwrap_or_else(|_| panic!("Failed to read the {:?} export file", &export_file_path));

  similar_asserts::assert_eq!(parsed_file.raw(), export_file);
}

#[rstest]
#[test]
fn verify_exported_files_match(
  #[files("tests/fixtures/archives/text/*.NOS")] archive_path: PathBuf,
) {
  let archive = Archive::from_file(&archive_path);
  assert!(
    archive.is_ok(),
    "Failed to parse the archive: {:?}",
    archive
  );
  let archive = archive.unwrap();

  for file in archive.files() {
    compare_files(file, &archive_path);
  }
}
