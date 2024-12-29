use rand::rngs::StdRng;
use rand::SeedableRng;
use std::cell::Cell;

thread_local! {
    static RAND_SEED: Cell<u64> = const { Cell::new(0) };
}

pub fn set_seed(seed: u64) {
  RAND_SEED.set(seed)
}

pub fn get_rng() -> StdRng {
  RAND_SEED.with(|seed| rand::rngs::StdRng::seed_from_u64(seed.get()))
}
