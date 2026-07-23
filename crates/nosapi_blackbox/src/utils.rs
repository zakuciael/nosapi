use crate::constants::VECTOR_STRING_LENGTH;
use rand::{
    Rng,
    distr::{Distribution, Uniform},
};

fn random_ascii_generator() -> Uniform<u8> {
    Uniform::new(32, 126).expect("This should be a valid ASCII char range")
}

pub fn rng_generator() -> impl Rng {
    #[cfg(test)]
    return crate::mock::rand::get_rng();

    #[cfg(not(test))]
    return rand::rng();
}

pub fn random_ascii_char() -> char {
    random_ascii_generator().sample(&mut rng_generator()).into()
}

pub fn random_ascii_string(length: usize) -> String {
    random_ascii_generator()
        .sample_iter(&mut rng_generator())
        .take(length)
        .map(char::from)
        .collect()
}

pub fn generate_vector_string() -> String {
    random_ascii_string(VECTOR_STRING_LENGTH)
}
