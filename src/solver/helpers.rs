use std::ops::Range;

use rand::{Rng, RngExt, SeedableRng, rngs::StdRng};

pub fn get_rng(maybe_seed: Option<u64>) -> impl Rng {
    if let Some(seed) = maybe_seed {
        StdRng::seed_from_u64(seed)
    } else {
        StdRng::from_rng(&mut rand::rng())
    }
}

pub fn select_idx_pair<R: Rng>(rng: &mut R, range: Range<usize>) -> (usize, usize) {
    let p1 = rng.random_range(range.clone());
    let mut p2 = rng.random_range(range.start..(range.end - 1));
    if p2 >= p1 {
        p2 += 1;
    }
    (p1, p2)
}

pub fn select_roulette<'a, R: Rng, T>(rng: &mut R, items: &'a [(T, f32)]) -> &'a T {
    let total_weight: f32 = items.iter().map(|&(_, w)| w).sum();
    let mut toss = rng.random_range(0.0..total_weight);
    for (item, weight) in items.iter() {
        toss -= weight;
        if toss <= 0.0 {
            return item;
        }
    }
    let (item, _) = items.last().unwrap();
    item
}
