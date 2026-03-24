use pfsp_solver::solver::{
    algorithm::operators::{InversionMutation, SwapMutation, UnaryOperator},
    evaluator::{Evaluator, TFTEvaluator},
    helpers::{get_rng, select_roulette},
    problem::Problem,
    solution::Solution,
};
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
const FIELD_MAX_FFE: &str = "max-ffe";

const DEFAULT_SWAP_MUTATION_P: f32 = 0.8;
const DEFAULT_INVERSION_MUTATION_P: f32 = 0.2;
const DEFAULT_MAX_FFE: u64 = 10_000;

define_algorithm!(AdapterRandom, "Random Search");

impl Default for AdapterRandom {
    fn default() -> Self {
        Self {
            settings: String::new(),
        }
    }
}

impl RunnableAdapter for AdapterRandom {
    fn run(&self, problem: &Problem, initial: Option<&Solution>, tx: UnboundedSender<RunLog>) {
        let settings = self.build_settings();
        let seed = get_optional_numeric_param(&settings, FIELD_SEED, problem.initial_seed);
        let mut rng = get_rng(seed);
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
        let weighted_ops = unary_ops
            .into_iter()
            .map(|op| {
                let p = op.probability();
                (op, p)
            })
            .collect::<Vec<_>>();
        let max_ffe = get_numeric_param(&settings, FIELD_MAX_FFE, DEFAULT_MAX_FFE);
        let mut current_solution = if let Some(initial_solution) = initial {
            initial_solution.clone()
        } else {
            Solution::random(&mut rng, problem.jobs_number)
        };

        let mut evaluator = TFTEvaluator::new(problem.processing_times.clone());
        let mut best_solution = current_solution.clone();
        let mut best_evaluation = evaluator.evaluate(&best_solution);

        while evaluator.eval_count() < max_ffe {
            let operator = select_roulette(&mut rng, &weighted_ops);
            operator.mutate(&mut rng, &mut current_solution);
            let current_evaluation = evaluator.evaluate(&current_solution);
            let accepted = current_evaluation < best_evaluation;
            if accepted {
                best_solution = current_solution.clone();
                best_evaluation = current_evaluation;
            }
            let message = format!(
                "operator: {}, fitness: {}, accepted: {}",
                operator.name(),
                current_evaluation,
                accepted
            );
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
