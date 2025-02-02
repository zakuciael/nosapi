use rand::Rng;

#[cfg(test)]
thread_local! {
    static SEED: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub fn set_seed(seed: u64) {
  SEED.set(seed)
}

pub fn get_rng() -> impl Rng {
  #[cfg(test)]
  {
    use rand::{rngs::StdRng, SeedableRng};
    SEED.with(|seed| StdRng::seed_from_u64(seed.get()))
  }

  #[cfg(not(test))]
  rand::thread_rng()
}
