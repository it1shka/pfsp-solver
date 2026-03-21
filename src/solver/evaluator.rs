use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::solver::{problem::Time, solution::Solution};

pub trait Evaluator {
    fn eval_count(&self) -> u64;
    fn evaluate(&mut self, solution: &Solution) -> Time;
}

pub struct TFTEvaluator<'a> {
    unique_evaluations: u64,
    cache: HashMap<u64, Time>,
    processing_times: &'a [Vec<Time>],
}

impl<'a> TFTEvaluator<'a> {
    pub fn new(processing_times: &'a [Vec<Time>]) -> Self {
        Self {
            unique_evaluations: 0,
            cache: HashMap::new(),
            processing_times,
        }
    }

    pub fn reset_cache(&mut self) {
        self.cache.clear();
    }
}

impl<'a> Evaluator for TFTEvaluator<'a> {
    fn eval_count(&self) -> u64 {
        self.unique_evaluations
    }

    fn evaluate(&mut self, solution: &Solution) -> Time {
        let mut hasher = DefaultHasher::new();
        solution.hash(&mut hasher);
        let key = hasher.finish();
        *self.cache.entry(key).or_insert_with(|| {
            self.unique_evaluations += 1;
            solution.total_flow_time(self.processing_times)
        })
    }
}
