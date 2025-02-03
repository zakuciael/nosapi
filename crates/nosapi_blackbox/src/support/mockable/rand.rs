use rand::Rng;

#[cfg(test)]
thread_local! {
    static SEED: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// A setter for the mock `seed` used in the [`get_rng`] function.
#[cfg(test)]
pub fn set_seed(seed: u64) {
  SEED.set(seed)
}

/// A function for returning a mockable random number generator.
pub fn get_rng() -> impl Rng {
  #[cfg(test)]
  {
    use rand::{rngs::StdRng, SeedableRng};
    SEED.with(|seed| StdRng::seed_from_u64(seed.get()))
  }

  #[cfg(not(test))]
  rand::thread_rng()
}
