use rand::distributions::{Distribution, Uniform};

const VECTOR_DATA_LENGTH: usize = 100;
const GAME_DATA_LENGTH: usize = 3;

fn random_ascii_generator() -> Uniform<u8> {
  Uniform::new(32, 126)
}

pub(crate) fn random_ascii_char() -> char {
  random_ascii_generator()
    .sample(&mut rand::thread_rng())
    .into()
}

pub(crate) fn random_ascii_string(length: usize) -> String {
  random_ascii_generator()
    .sample_iter(&mut rand::thread_rng())
    .take(length)
    .map(char::from)
    .collect()
}

pub fn generate_vector_data() -> String {
  random_ascii_string(VECTOR_DATA_LENGTH)
}

pub fn generate_game_data() -> String {
  random_ascii_string(GAME_DATA_LENGTH)
}
