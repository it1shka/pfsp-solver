use rand::seq::SliceRandom;

use crate::solver::{helpers::get_rng, solution::Solution};

pub struct Population(pub Vec<Solution>);

impl Population {
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn new(value: Vec<Solution>) -> Self {
        Self(value)
    }

    pub fn random(population_size: usize, jobs_amount: usize, maybe_seed: Option<u64>) -> Self {
        let mut rng = get_rng(maybe_seed);
        let mut range = (0..jobs_amount).collect::<Vec<usize>>();
        Self(
            (0..population_size)
                .map(|_| {
                    range.shuffle(&mut rng);
                    Solution::new(range.clone())
                })
                .collect(),
        )
    }
}
