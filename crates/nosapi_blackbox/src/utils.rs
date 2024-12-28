use rand::distributions::{Distribution, Uniform};

const VECTOR_STRING_LENGTH: usize = 100;
const GAME_STRING_LENGTH: usize = 3;

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

pub fn generate_vector_string() -> String {
  random_ascii_string(VECTOR_STRING_LENGTH)
}

pub fn generate_game_string() -> String {
  random_ascii_string(GAME_STRING_LENGTH)
}