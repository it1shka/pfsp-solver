use std::cmp::Reverse;

use rand::Rng;

use crate::solver::{
    algorithm::operators::{BinaryOperator, UnaryOperator},
    population::Population,
    problem::Time,
    solution::Solution,
};

pub struct GeneticAlgorithm<R: Rng> {
    rng: R,
    evaluate: fn(&Solution) -> Time,
    eval_cache: Vec<Time>,
    next_population: Population,
    pub population: Population,
    pub selection: Selection,
    pub elite_p: f32,
    pub binary_ops: Vec<Box<dyn BinaryOperator<R>>>,
    pub unary_ops: Vec<Box<dyn UnaryOperator<R>>>,
}

#[derive(Clone, Copy)]
pub enum Selection {
    Tournament(usize),
    Roulette,
}

#[derive(Clone, Copy)]
pub struct EvolutionStats {}

impl EvolutionStats {}

impl<R: Rng> GeneticAlgorithm<R> {
    pub fn new(
        rng: R,
        evaluate: fn(&Solution) -> Time,
        population: Population,
        selection: Selection,
        elite_p: f32,
        binary_ops: Vec<Box<dyn BinaryOperator<R>>>,
        unary_ops: Vec<Box<dyn UnaryOperator<R>>>,
    ) -> Self {
        Self {
            rng: rng,
            evaluate: evaluate,
            eval_cache: vec![],
            next_population: Population::empty(),
            population: population,
            selection: selection,
            elite_p: elite_p,
            binary_ops: binary_ops,
            unary_ops: unary_ops,
        }
    }

    fn evaluate_population(&mut self) {
        self.eval_cache.clear();
        for s in &self.population.data {
            let evaluation = (self.evaluate)(s);
            self.eval_cache.push(evaluation);
        }
    }

    fn population_iter(&self) -> impl Iterator<Item = (&Solution, Time)> {
        self.population
            .data
            .iter()
            .enumerate()
            .map(|(i, s)| (s, self.eval_cache[i]))
    }

    pub fn evolution_cycle(&mut self) {
        self.evaluate_population();
        self.select_elite();
        self.select_parents();
        self.perform_crossovers();
        self.perform_mutations();
        self.swap_populations();
    }

    fn select_elite(&mut self) {
        let elite_count = (self.elite_p * self.population.len() as f32).round() as usize;
        let elite = {
            let mut rated_population = self.population_iter().collect::<Vec<_>>();
            rated_population.sort_by_key(|&(_, time)| Reverse(time));
            rated_population
                .into_iter()
                .take(elite_count)
                .map(|(s, _)| s.clone())
                .collect::<Vec<_>>()
        };
        self.next_population.extend(elite);
    }

    fn select_parents(&mut self) {}

    fn perform_crossovers(&mut self) {}

    fn perform_mutations(&mut self) {}

    fn swap_populations(&mut self) {}
}
