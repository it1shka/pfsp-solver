use rand::{SeedableRng, rngs::StdRng};

pub fn get_rng(maybe_seed: Option<u64>) -> StdRng {
    if let Some(seed) = maybe_seed {
        StdRng::seed_from_u64(seed)
    } else {
        StdRng::from_rng(&mut rand::rng())
    }
}
