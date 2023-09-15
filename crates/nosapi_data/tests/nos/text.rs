use std::fs::File;

use color_eyre::eyre::WrapErr;
use memmap::Mmap;
use sha1_smol::Sha1;

use nosapi_data::nos::error::NOSFileError;
use nosapi_data::nos::{NOSTextFile, NOSTextFileEntry};

static FILE_PATH: &str = "./tests/fixtures/NSgtdData.NOS";
static LST_FILE_SHA1: &str = "da39a3ee5e6b4b0d3255bfef95601890afd80709";
static DAT_FILE_SHA1_LIST: [(&str, &str); 28] = [
  ("34437ed9eb8c48520c3b4e46193b5d4db543a530", "act_desc.dat"),
  ("6c6534e2f9c3ddd2c8dde36e7d385bfbf3603604", "BCard.dat"),
  ("80ec2b005f87c8dd1407a6345b25aee8c3fc5b22", "Card.dat"),
  ("9d4cb24fe198242f37261fbd8c45d1d0d627615c", "cz_nosmall.dat"),
  ("9d4cb24fe198242f37261fbd8c45d1d0d627615c", "de_nosmall.dat"),
  ("9d4cb24fe198242f37261fbd8c45d1d0d627615c", "es_nosmall.dat"),
  ("cc454413b19f20cef44a5b5c8e7d1e63c23d16d7", "fish.dat"),
  ("9d4cb24fe198242f37261fbd8c45d1d0d627615c", "fr_nosmall.dat"),
  ("744135b8dbc1a532a99785d4d3061c043889c2d5", "hk_nosmall.dat"),
  ("9d4cb24fe198242f37261fbd8c45d1d0d627615c", "it_nosmall.dat"),
  ("d05fa343531bdf04445deb1b6cd725275addaa60", "Item.dat"),
  ("78fc3ef43ad31277ee55007949d023f75fc97736", "kr_nosmall.dat"),
  ("e0ac44650e5cf0c2773dd5cda0df655b48c19769", "MapIDData.dat"),
  (
    "955d200eda801a42d826af9b3426e54ff08c42e1",
    "MapPointData.dat",
  ),
  ("3640ae34900dcb8603db3d13a21ab39f93d4c536", "monster.dat"),
  ("917a68f6d8f648d45c1e38eedd2f826101d92803", "npctalk.dat"),
  ("9d4cb24fe198242f37261fbd8c45d1d0d627615c", "pl_nosmall.dat"),
  ("278e05a0ce579dfbec87594cdef93fc85468e4c7", "qstnpc.dat"),
  ("b0a894862ea43c123770af1a0d307e90e258a314", "qstprize.dat"),
  ("ba5b316bc5721d33e2b7b78ba40fe300b65b63ce", "quest.dat"),
  ("9d4cb24fe198242f37261fbd8c45d1d0d627615c", "ru_nosmall.dat"),
  ("8c3c52e87d7a51f080e51f31685cdeb65a24050f", "shoptype.dat"),
  ("6422d44a459eff9b7073582389e465b79675315e", "Skill.dat"),
  ("853ffa67c6f414f0b8d32941e1e4880570644986", "team.dat"),
  ("9d4cb24fe198242f37261fbd8c45d1d0d627615c", "tr_nosmall.dat"),
  ("1b2b32cce55ce0715db076f44201e1c7cf0ff356", "tutorial.dat"),
  ("9d4cb24fe198242f37261fbd8c45d1d0d627615c", "uk_nosmall.dat"),
  ("c31395ba4ce16658ee5c6b3e40c948da049c2a92", "hk_abuse.lst"),
];

fn load_file(path: &str) -> Result<NOSTextFile, NOSFileError> {
  let file = File::open(path)?;
  let mmap = unsafe { Mmap::map(&file)? };
  NOSTextFile::from_bytes(&mmap)
}

#[test]
fn correctly_parses_ole_time() {
  let file = load_file(FILE_PATH)
    .wrap_err_with(|| format!("Failed to parse file {}", &FILE_PATH))
    .unwrap();

  assert!(file.ole_time.is_some());
  assert_eq!(
    file.ole_time.unwrap().to_string(),
    "2023-08-23 15:27:04 UTC"
  )
}

#[test]
fn correctly_parses_entry_metadata() {
  let file = load_file(FILE_PATH)
    .wrap_err_with(|| format!("Failed to parse file {}", &FILE_PATH))
    .unwrap();

  let entry = file
    .files
    .first()
    .expect("Failed to parse file entries, no entries found.");
  assert_eq!(
    entry,
    &NOSTextFileEntry {
      file_number: 1,
      name: "kr_abuse.lst".to_owned(),
      is_dat: false,
      size: 0,
      raw_content: vec![]
    }
  )
}

#[test]
fn correctly_decrypts_files() {
  let file = load_file(FILE_PATH)
    .wrap_err_with(|| format!("Failed to parse file {}", &FILE_PATH))
    .unwrap();

  for entry in &file.files {
    let result_sha1 = Sha1::from(&entry.raw_content).hexdigest();
    let expected_sha1 = if entry.name.ends_with(".lst") && entry.name != "hk_abuse.lst" {
      LST_FILE_SHA1
    } else {
      DAT_FILE_SHA1_LIST
        .iter()
        .find(|(_, file_name)| file_name == &entry.name)
        .map(|(sha1, _)| sha1.to_owned())
        .unwrap()
    };

    assert_eq!(
      result_sha1, expected_sha1,
      "File {} doesn't match the expected sha1 checksum.",
      entry.name
    )
  }
}
