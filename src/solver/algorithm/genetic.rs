use rand::Rng;

use crate::solver::{
    algorithm::operators::{BinaryOperator, UnaryOperator},
    population::Population,
};

pub struct GeneticAlgorithm<R: Rng> {
    pub rng: R,
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

impl<R: Rng> GeneticAlgorithm<R> {
    pub fn evolution_cycle(&mut self) {
        todo!()
    }
}
