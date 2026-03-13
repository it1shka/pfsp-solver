use rand::{Rng, seq::SliceRandom};

use crate::solver::solution::Solution;

pub struct Population {
    pub data: Vec<Solution>,
}

impl Population {
    pub fn empty() -> Self {
        Self { data: vec![] }
    }

    pub fn new(value: Vec<Solution>) -> Self {
        Self { data: value }
    }

    pub fn random<R: Rng>(rng: &mut R, population_size: usize, jobs_amount: usize) -> Self {
        let mut range = (0..jobs_amount).collect::<Vec<usize>>();
        Self {
            data: (0..population_size)
                .map(|_| {
                    range.shuffle(rng);
                    Solution::new(range.clone())
                })
                .collect(),
        }
    }
}
