use std::cmp::max;

use pfsp_solver::solver::{
    algorithm::{
        genetic::GeneticAlgorithm,
        operators::{
            BinaryOperator, CycleCrossover, InversionMutation, OrderedCrossover,
            PartiallyMatchedCrossover, SwapMutation, UnaryOperator,
        },
        selectors::{Roulette, Selector, Tournament},
    },
    evaluator::TFTEvaluator,
    helpers::get_rng,
    population::{MIN_POPULATION_SIZE, Population},
    problem::{Problem, Time},
    solution::Solution,
};
use rand::Rng;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    define_algorithm,
    tui::adapters::{
        adapter::{Adapter, RunLog, RunnableAdapter, Settings},
        helpers::{add_binary_op, add_unary_op, get_numeric_param, get_optional_numeric_param},
    },
};

const FIELD_SEED: &str = "seed";
const FIELD_POPULATION_SIZE: &str = "population-size";
const FIELD_SELECTOR: &str = "selector";
const FIELD_TOURNAMENT_SIZE: &str = "tournament-size";
const FIELD_ELITE_P: &str = "elitism";
const FIELD_ORDERED_CROSSOVER_P: &str = "ordered-crossover";
const FIELD_PARTIALLY_MATCHED_CROSSOVER_P: &str = "partially-matched-crossover";
const FIELD_CYCLE_CROSSOVER_P: &str = "cycle-crossover";
const FIELD_SWAP_MUTATION_P: &str = "swap-mutation";
const FIELD_INVERSION_MUTATION_P: &str = "inversion-mutation";
const FIELD_MAX_FFE: &str = "max-ffe";

const DEFAULT_POPULATION_SIZE: usize = 1000;
const DEFAULT_SELECTOR: &str = "roulette";
const DEFAULT_TOURNAMENT_SIZE: usize = 5;
const DEFAULT_ELITE_P: f32 = 0.05;
const DEFAULT_ORDERED_CROSSOVER_P: f32 = 0.1;
const DEFAULT_PARTIALLY_MATCHED_CROSSOVER_P: f32 = 0.1;
const DEFAULT_CYCLE_CROSSOVER_P: f32 = 0.1;
const DEFAULT_SWAP_MUTATION_P: f32 = 0.05;
const DEFAULT_INVERSION_MUTATION_P: f32 = 0.05;
const DEFAULT_MAX_FFE: u64 = 10_000;

const SELECTOR_TOURNAMENT: &str = "tournament";

define_algorithm!(AdapterGA, "Genetic Algorithm");

impl Default for AdapterGA {
    fn default() -> Self {
        let settings = [
            (FIELD_POPULATION_SIZE, DEFAULT_POPULATION_SIZE.to_string()),
            (FIELD_SELECTOR, DEFAULT_SELECTOR.to_string()),
            (FIELD_ELITE_P, DEFAULT_ELITE_P.to_string()),
            (
                FIELD_ORDERED_CROSSOVER_P,
                DEFAULT_ORDERED_CROSSOVER_P.to_string(),
            ),
            (
                FIELD_PARTIALLY_MATCHED_CROSSOVER_P,
                DEFAULT_PARTIALLY_MATCHED_CROSSOVER_P.to_string(),
            ),
            (
                FIELD_CYCLE_CROSSOVER_P,
                DEFAULT_CYCLE_CROSSOVER_P.to_string(),
            ),
            (FIELD_SWAP_MUTATION_P, DEFAULT_SWAP_MUTATION_P.to_string()),
            (
                FIELD_INVERSION_MUTATION_P,
                DEFAULT_INVERSION_MUTATION_P.to_string(),
            ),
            (FIELD_MAX_FFE, DEFAULT_MAX_FFE.to_string()),
        ]
        .into_iter()
        .map(|(field, value)| format!("{}: {}", field, value))
        .collect::<Vec<_>>()
        .join("\n");
        Self { settings }
    }
}

impl AdapterGA {
    fn configure_genetic(
        &self,
        problem: &Problem,
        initial: Option<&Solution>,
    ) -> GeneticAlgorithm<impl Rng> {
        let settings = self.build_settings();
        let seed = get_optional_numeric_param(&settings, FIELD_SEED, problem.initial_seed);
        let mut rng = get_rng(seed);
        let population_size = {
            let initial_population_size =
                get_numeric_param(&settings, FIELD_POPULATION_SIZE, DEFAULT_POPULATION_SIZE);
            max(MIN_POPULATION_SIZE, initial_population_size)
        };
        let population = if let Some(initial_solution) = initial {
            Population::from_initial(&mut rng, population_size, initial_solution)
        } else {
            Population::random(&mut rng, population_size, problem.jobs_number)
        };
        let selector: Box<dyn Selector<_>> = {
            if let Some(evaluator_type) = settings.get(FIELD_SELECTOR)
                && evaluator_type == SELECTOR_TOURNAMENT
            {
                let tournament_size =
                    get_numeric_param(&settings, FIELD_TOURNAMENT_SIZE, DEFAULT_TOURNAMENT_SIZE);
                Box::new(Tournament::new(tournament_size))
            } else {
                Box::new(Roulette::new())
            }
        };
        let elite_p = get_numeric_param(&settings, FIELD_ELITE_P, DEFAULT_ELITE_P);
        let binary_ops = {
            let mut binary_ops: Vec<Box<dyn BinaryOperator<_>>> = vec![];
            add_binary_op::<OrderedCrossover, _>(
                &settings,
                &mut binary_ops,
                FIELD_ORDERED_CROSSOVER_P,
                DEFAULT_ORDERED_CROSSOVER_P,
            );
            add_binary_op::<PartiallyMatchedCrossover, _>(
                &settings,
                &mut binary_ops,
                FIELD_PARTIALLY_MATCHED_CROSSOVER_P,
                DEFAULT_PARTIALLY_MATCHED_CROSSOVER_P,
            );
            add_binary_op::<CycleCrossover, _>(
                &settings,
                &mut binary_ops,
                FIELD_CYCLE_CROSSOVER_P,
                DEFAULT_CYCLE_CROSSOVER_P,
            );
            binary_ops
        };
        let unary_ops = {
            let mut unary_ops: Vec<Box<dyn UnaryOperator<_>>> = vec![];
            add_unary_op::<SwapMutation, _>(
                &settings,
                &mut unary_ops,
                FIELD_SWAP_MUTATION_P,
                DEFAULT_SWAP_MUTATION_P,
            );
            add_unary_op::<InversionMutation, _>(
                &settings,
                &mut unary_ops,
                FIELD_INVERSION_MUTATION_P,
                DEFAULT_INVERSION_MUTATION_P,
            );
            unary_ops
        };
        let evaluator = Box::new(TFTEvaluator::new(problem.processing_times.clone()));
        GeneticAlgorithm::new(
            rng, population, evaluator, selector, elite_p, binary_ops, unary_ops,
        )
    }
}

impl RunnableAdapter for AdapterGA {
    fn run(&self, problem: &Problem, initial: Option<&Solution>, tx: UnboundedSender<RunLog>) {
        let settings = self.build_settings();
        let max_ffe = get_numeric_param(&settings, FIELD_MAX_FFE, DEFAULT_MAX_FFE);
        let mut genetic = self.configure_genetic(problem, initial);
        let mut best_solution = genetic.population.data[0].clone();
        let mut best_evaluation = Time::MAX;
        while genetic.evaluator.eval_count() < max_ffe {
            genetic.evolution_cycle();
            let best_from_generation = genetic
                .population
                .data
                .iter()
                .min_by_key(|&s| genetic.evaluator.evaluate(s))
                .unwrap()
                .clone();
            let best_from_generation_evaluation = genetic.evaluator.evaluate(&best_from_generation);
            if best_from_generation_evaluation < best_evaluation {
                best_solution = best_from_generation;
                best_evaluation = best_from_generation_evaluation;
            }
            let message = genetic
                .stats
                .operators_usage
                .iter()
                .map(|(&name, &usage)| format!("{} usage: {}", name, usage))
                .collect::<Vec<_>>()
                .join(", ");
            let result = tx.send(RunLog {
                best: best_solution.clone(),
                fitness: best_evaluation,
                message,
            });
            if result.is_err() {
                break;
            }
        }
    }
}
