use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::solver::{problem::Time, solution::Solution};

pub trait Evaluator {
    fn evaluate(&mut self, solution: &Solution) -> Time;
}

pub struct TFTEvaluator<'a> {
    cache: HashMap<u64, Time>,
    processing_times: &'a [Vec<Time>],
}

impl<'a> TFTEvaluator<'a> {
    pub fn new(processing_times: &'a [Vec<Time>]) -> Self {
        Self {
            cache: HashMap::new(),
            processing_times,
        }
    }

    pub fn reset_cache(&mut self) {
        self.cache.clear();
    }
}

impl<'a> Evaluator for TFTEvaluator<'a> {
    fn evaluate(&mut self, solution: &Solution) -> Time {
        let mut hasher = DefaultHasher::new();
        solution.hash(&mut hasher);
        let key = hasher.finish();
        *self
            .cache
            .entry(key)
            .or_insert_with(|| solution.total_flow_time(self.processing_times))
    }
}
