use rand::distributions::{Distribution, Uniform};
use std::sync::LazyLock;

const VECTOR_STRING_DATA_LENGTH: usize = 100;

static ASCII_DISTRIBUTION: LazyLock<Uniform<u8>> = LazyLock::new(|| Uniform::new(32, 126));

pub(super) fn random_ascii_char() -> char {
  let mut rng = crate::support::mockable::rand::get_rng();

  ASCII_DISTRIBUTION.sample(&mut rng) as char
}

pub(super) fn random_vector_string_data() -> String {
  let mut rng = crate::support::mockable::rand::get_rng();

  ASCII_DISTRIBUTION
    .sample_iter(&mut rng)
    .take(VECTOR_STRING_DATA_LENGTH)
    .map(char::from)
    .collect()
}
