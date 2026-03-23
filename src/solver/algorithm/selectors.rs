use rand::{Rng, RngExt, seq::IndexedRandom};

use crate::solver::{evaluator::Evaluator, population::Population, solution::Solution};

pub trait Selector<R: Rng> {
    fn name(&self) -> &'static str;
    fn select<'a: 'b, 'b>(
        &self,
        rng: &mut R,
        population: &'a Population,
        evaluator: &mut dyn Evaluator,
    ) -> &'b Solution;
}

pub struct Tournament {
    pub size: usize,
}

impl Tournament {
    pub fn new(size: usize) -> Self {
        Self { size }
    }
}

impl<R: Rng> Selector<R> for Tournament {
    fn name(&self) -> &'static str {
        "Tournament"
    }

    fn select<'a: 'b, 'b>(
        &self,
        rng: &mut R,
        population: &'a Population,
        evaluator: &mut dyn Evaluator,
    ) -> &'b Solution {
        let pool = population.data.sample(rng, self.size);
        let best = pool.min_by_key(|&s| evaluator.evaluate(s));
        best.unwrap()
    }
}

pub struct Roulette;

impl Roulette {
    pub fn new() -> Self {
        Self {}
    }
}

impl<R: Rng> Selector<R> for Roulette {
    fn name(&self) -> &'static str {
        "Roulette"
    }

    fn select<'a: 'b, 'b>(
        &self,
        rng: &mut R,
        population: &'a Population,
        evaluator: &mut dyn Evaluator,
    ) -> &'b Solution {
        let fitness = population
            .data
            .iter()
            .map(|s| 1.0 / (evaluator.evaluate(s) as f32 + 1e-6))
            .collect::<Vec<_>>();
        let total_fitness: f32 = fitness.iter().sum();
        let mut toss = rng.random_range(0.0..total_fitness);

        for (i, &f) in fitness.iter().enumerate() {
            toss -= f;
            if toss <= 0.0 {
                return &population.data[i];
            }
        }
        population.data.last().unwrap()
    }
}
