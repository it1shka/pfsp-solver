use std::{cmp::Reverse, collections::HashMap, mem};

use rand::{Rng, seq::IndexedMutRandom};

use crate::solver::{
    algorithm::{
        operators::{BinaryOperator, UnaryOperator},
        selectors::Selector,
    },
    helpers::select_idx_pair,
    population::Population,
    problem::Time,
    solution::Solution,
};

pub struct EvolutionStats {
    pub operators_usage: HashMap<&'static str, usize>,
    pub best_time: Time,
}

impl EvolutionStats {
    fn new() -> Self {
        Self {
            operators_usage: HashMap::new(),
            best_time: Time::MAX,
        }
    }

    fn clear(&mut self) {
        self.operators_usage.clear();
        self.best_time = Time::MAX;
    }

    fn increase_operator_usage(&mut self, name: &'static str, usage: usize) {
        let entry = self.operators_usage.entry(name).or_insert(0);
        *entry += usage;
    }
}

pub struct GeneticAlgorithm<R: Rng> {
    rng: R,
    evaluate: fn(&Solution) -> Time,
    eval_cache: Vec<Time>,
    next_population: Population,
    stats: EvolutionStats,
    pub population: Population,
    pub selection: Box<dyn Selector<R>>,
    pub elite_p: f32,
    pub binary_ops: Vec<Box<dyn BinaryOperator<R>>>,
    pub unary_ops: Vec<Box<dyn UnaryOperator<R>>>,
}

impl<R: Rng> GeneticAlgorithm<R> {
    pub fn new(
        rng: R,
        evaluate: fn(&Solution) -> Time,
        population: Population,
        selection: Box<dyn Selector<R>>,
        elite_p: f32,
        binary_ops: Vec<Box<dyn BinaryOperator<R>>>,
        unary_ops: Vec<Box<dyn UnaryOperator<R>>>,
    ) -> Self {
        Self {
            rng: rng,
            evaluate: evaluate,
            eval_cache: vec![],
            next_population: Population::empty(),
            stats: EvolutionStats::new(),
            population: population,
            selection: selection,
            elite_p: elite_p,
            binary_ops: binary_ops,
            unary_ops: unary_ops,
        }
    }

    // helper functions

    fn population_iter(&self) -> impl Iterator<Item = (&Solution, Time)> {
        self.population
            .data
            .iter()
            .enumerate()
            .map(|(i, s)| (s, self.eval_cache[i]))
    }

    // main cycle

    pub fn evolution_cycle(&mut self) {
        self.reset_before_evolution_cycle();
        self.evaluate_population();
        self.select_elite();
        self.select_parents();
        self.perform_binary_ops();
        self.perform_unary_ops();
        self.swap_populations();
    }

    fn reset_before_evolution_cycle(&mut self) {
        self.eval_cache.clear();
        self.next_population.clear();
        self.stats.clear();
    }

    fn evaluate_population(&mut self) {
        for s in &self.population.data {
            let evaluation = (self.evaluate)(s);
            self.eval_cache.push(evaluation);
        }
    }

    fn select_elite(&mut self) {
        let elite_count = self.population.p_count(self.elite_p);
        let elite = {
            let mut rated_population = self.population_iter().collect::<Vec<_>>();
            rated_population.sort_by_key(|&(_, time)| time);
            rated_population
                .into_iter()
                .take(elite_count)
                .map(|(s, _)| s.clone())
                .collect::<Vec<_>>()
        };
        self.next_population.extend(elite);
    }

    fn select_parents(&mut self) {
        while self.next_population.len() < self.population.len() {
            let parent = self
                .selection
                .select(&mut self.rng, &mut self.population, self.evaluate);
            self.next_population.push(parent.clone());
        }
    }

    fn perform_binary_ops(&mut self) {
        self.binary_ops.iter().for_each(|op| {
            let effect_count = self.next_population.p_count(op.probability());
            for _ in 0..effect_count {
                let (idx1, idx2) = select_idx_pair(&mut self.rng, 0..self.next_population.len());

                // More readable but not idiomatic
                /*
                unsafe {
                    let population = self.next_population.data.as_mut_ptr();
                    let p1 = &mut *population.add(idx1);
                    let p2 = &*population.add(idx2);
                    op.mutate(&mut self.rng, p1, p2);
                }
                */

                let data = &mut self.next_population.data;
                let (p1, p2) = if idx1 < idx2 {
                    let (left, right) = data.split_at_mut(idx2);
                    (&mut left[idx1], &right[0])
                } else {
                    let (left, right) = data.split_at_mut(idx1);
                    (&mut right[0], &left[idx2])
                };
                op.mutate(&mut self.rng, p1, p2);
            }
            self.stats.increase_operator_usage(op.name(), effect_count);
        });
    }

    fn perform_unary_ops(&mut self) {
        self.unary_ops.iter().for_each(|op| {
            let effect_count = self.next_population.p_count(op.probability());
            for _ in 0..effect_count {
                let p = self.next_population.data.choose_mut(&mut self.rng).unwrap();
                op.mutate(&mut self.rng, p);
            }
            self.stats.increase_operator_usage(op.name(), effect_count);
        });
    }

    fn swap_populations(&mut self) {
        self.population = mem::replace(&mut self.next_population, Population::empty());
        let best_result = self
            .population
            .data
            .iter()
            .map(|s| (self.evaluate)(s))
            .min()
            .unwrap();
        self.stats.best_time = best_result;
    }
}
