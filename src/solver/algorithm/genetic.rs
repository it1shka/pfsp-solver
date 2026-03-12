use crate::solver::{
    algorithm::operators::{BinaryOperator, UnaryOperator},
    population::Population,
    problem::Time,
    solution::Solution,
};

pub struct GeneticAlgorithm {
    pub population: Population,
}

pub struct EvolutionParams {
    pub selection: Selection,
    pub elite_p: f32,
    pub bin_ops: Vec<Box<dyn BinaryOperator>>,
    pub un_ops: Vec<Box<dyn UnaryOperator>>,
}

#[derive(Clone, Copy)]
pub enum Selection {
    Tournament(usize),
    Roulette,
}

impl GeneticAlgorithm {
    pub fn init_with_solution(init_solution: &Solution, population_size: usize) -> Self {
        // TODO: transform the solution multiple times and store all the altered versions together
        todo!()
    }

    pub fn init_with_population(init_population: Population) -> Self {
        Self {
            population: init_population,
        }
    }

    pub fn evolution_cycle(&mut self, params: &EvolutionParams) {}
}
