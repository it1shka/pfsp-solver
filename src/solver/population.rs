use rand::{Rng, seq::SliceRandom};

use crate::solver::solution::Solution;

pub struct Population {
    pub data: Vec<Solution>,
}

pub const MIN_POPULATION_SIZE: usize = 4;

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

    pub fn is_valid(&self) -> bool {
        self.len() >= MIN_POPULATION_SIZE
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, solution: Solution) {
        self.data.push(solution);
    }

    pub fn extend(&mut self, batch: Vec<Solution>) {
        self.data.extend(batch);
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn p_count(&self, p: f32) -> usize {
        (p * self.len() as f32).round() as usize
    }
}
