use pfsp_solver::solver::{
    algorithm::{
        annealing::SimulatedAnnealing,
        operators::{InversionMutation, SwapMutation, UnaryOperator},
    },
    evaluator::TFTEvaluator,
    helpers::get_rng,
    problem::Problem,
    solution::Solution,
};
use rand::Rng;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    define_algorithm,
    tui::adapters::{
        adapter::{Adapter, RunLog, RunnableAdapter, Settings},
        helpers::{add_unary_op, get_numeric_param, get_optional_numeric_param},
    },
};

const FIELD_SEED: &str = "seed";
const FIELD_SWAP_MUTATION_P: &str = "swap-mutation";
const FIELD_INVERSION_MUTATION_P: &str = "inversion-mutation";
const FIELD_TEMPERATURE: &str = "temperature";
const FIELD_DECAY: &str = "decay";
const FIELD_THRESHOLD: &str = "threshold";

const DEFAULT_SWAP_MUTATION_P: f32 = 0.8;
const DEFAULT_INVERSION_MUTATION_P: f32 = 0.2;
const DEFAULT_TEMPERATURE: f64 = 100.0;
const DEFAULT_DECAY: f64 = 0.90;
const DEFAULT_THRESHOLD: f64 = 0.1;

define_algorithm!(AdapterAnnealing, "Simulated Annealing");

impl Default for AdapterAnnealing {
    fn default() -> Self {
        let settings = [
            (FIELD_SWAP_MUTATION_P, DEFAULT_SWAP_MUTATION_P.to_string()),
            (
                FIELD_INVERSION_MUTATION_P,
                DEFAULT_INVERSION_MUTATION_P.to_string(),
            ),
            (FIELD_TEMPERATURE, DEFAULT_TEMPERATURE.to_string()),
            (FIELD_DECAY, DEFAULT_DECAY.to_string()),
            (FIELD_THRESHOLD, DEFAULT_THRESHOLD.to_string()),
        ]
        .into_iter()
        .map(|(field, value)| format!("{}: {}", field, value))
        .collect::<Vec<_>>()
        .join("\n");

        Self { settings }
    }
}

impl AdapterAnnealing {
    fn configure_annealing(
        &self,
        problem: &Problem,
        initial: Option<&Solution>,
    ) -> SimulatedAnnealing<impl Rng> {
        let settings = self.build_settings();
        let seed = get_optional_numeric_param(&settings, FIELD_SEED, problem.initial_seed);
        let mut rng = get_rng(seed);
        let solution = if let Some(initial_solution) = initial {
            initial_solution.clone()
        } else {
            Solution::random(&mut rng, problem.jobs_number)
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
        let temperature = get_numeric_param(&settings, FIELD_TEMPERATURE, DEFAULT_TEMPERATURE);
        let decay = get_numeric_param(&settings, FIELD_DECAY, DEFAULT_DECAY);
        let threshold = get_numeric_param(&settings, FIELD_THRESHOLD, DEFAULT_THRESHOLD);
        let evaluator = Box::new(TFTEvaluator::new(problem.processing_times.clone()));

        SimulatedAnnealing::new(
            rng,
            solution,
            evaluator,
            unary_ops,
            temperature,
            decay,
            threshold,
        )
    }
}

impl RunnableAdapter for AdapterAnnealing {
    async fn run(
        &self,
        _problem: &Problem,
        _initial: Option<&Solution>,
        _tx: UnboundedSender<RunLog>,
    ) {
    }
}
