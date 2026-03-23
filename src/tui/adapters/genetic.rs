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
    problem::Problem,
    solution::Solution,
};
use rand::Rng;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    define_algorithm,
    tui::adapters::adapter::{Adapter, RunLog, RunnableAdapter, Settings},
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

const DEFAULT_POPULATION_SIZE: usize = 1000;
const DEFAULT_TOURNAMENT_SIZE: usize = 5;
const DEFAULT_ELITE_P: f32 = 0.05;
const DEFAULT_ORDERED_CROSSOVER_P: f32 = 0.1;
const DEFAULT_PARTIALLY_MATCHED_CROSSOVER_P: f32 = 0.1;
const DEFAULT_CYCLE_CROSSOVER_P: f32 = 0.1;
const DEFAULT_SWAP_MUTATION_P: f32 = 0.05;
const DEFAULT_INVERSION_MUTATION_P: f32 = 0.05;

const SELECTOR_TOURNAMENT: &str = "tournament";

define_algorithm!(AdapterGA, "Genetic Algorithm");

impl AdapterGA {
    fn configure_genetic(
        &self,
        problem: &Problem,
        initial: Option<Solution>,
    ) -> GeneticAlgorithm<impl Rng> {
        let settings = self.build_settings();
        macro_rules! get_numeric_param {
            ($type:ty,$field:expr,$default:expr) => {
                settings
                    .get($field)
                    .map(|raw| raw.parse::<$type>().ok())
                    .flatten()
                    .unwrap_or($default)
            };
        }
        macro_rules! add_operator {
            ($container:expr,$op_struct:ident,$field:expr,$default:expr) => {
                let probability = get_numeric_param!(f32, $field, $default);
                // if (probability > 0.0) {
                //     $container.push(Box::new($op_struct { p: probability }))
                // }
                $container.push(Box::new($op_struct::new(probability)))
            };
        }

        let seed = {
            settings
                .get(FIELD_SEED)
                .map(|raw| raw.parse::<u64>().ok())
                .flatten()
                .or(problem.initial_seed)
        };
        let mut rng = get_rng(seed);
        let population_size = {
            let initial_population_size =
                get_numeric_param!(usize, FIELD_POPULATION_SIZE, DEFAULT_POPULATION_SIZE);
            max(MIN_POPULATION_SIZE, initial_population_size)
        };
        let population = if let Some(initial_solution) = initial {
            Population::from_initial(&mut rng, population_size, &initial_solution)
        } else {
            Population::random(&mut rng, population_size, problem.jobs_number)
        };
        let selector: Box<dyn Selector<_>> = {
            if let Some(evaluator_type) = settings.get(FIELD_SELECTOR)
                && evaluator_type == SELECTOR_TOURNAMENT
            {
                let tournament_size =
                    get_numeric_param!(usize, FIELD_TOURNAMENT_SIZE, DEFAULT_TOURNAMENT_SIZE);
                Box::new(Tournament::new(tournament_size))
            } else {
                Box::new(Roulette::new())
            }
        };
        let elite_p = get_numeric_param!(f32, FIELD_ELITE_P, DEFAULT_ELITE_P);
        let binary_ops = {
            let mut binary_ops: Vec<Box<dyn BinaryOperator<_>>> = vec![];
            add_operator!(
                binary_ops,
                OrderedCrossover,
                FIELD_ORDERED_CROSSOVER_P,
                DEFAULT_ORDERED_CROSSOVER_P
            );
            add_operator!(
                binary_ops,
                PartiallyMatchedCrossover,
                FIELD_PARTIALLY_MATCHED_CROSSOVER_P,
                DEFAULT_PARTIALLY_MATCHED_CROSSOVER_P
            );
            add_operator!(
                binary_ops,
                CycleCrossover,
                FIELD_CYCLE_CROSSOVER_P,
                DEFAULT_CYCLE_CROSSOVER_P
            );
            binary_ops
        };
        let unary_ops = {
            let mut unary_ops: Vec<Box<dyn UnaryOperator<_>>> = vec![];
            add_operator!(
                unary_ops,
                SwapMutation,
                FIELD_SWAP_MUTATION_P,
                DEFAULT_SWAP_MUTATION_P
            );
            add_operator!(
                unary_ops,
                InversionMutation,
                FIELD_INVERSION_MUTATION_P,
                DEFAULT_INVERSION_MUTATION_P
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
    async fn run(&self, problem: &Problem, initial: Option<Solution>, tx: UnboundedSender<RunLog>) {
    }
}
