pub fn main() {
  println!("cargo::rerun-if-changed=tests/fixtures");
  println!("cargo::rerun-if-env-changed=BASE_TEST_DIR");
}
