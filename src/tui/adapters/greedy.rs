use pfsp_solver::solver::{
    algorithm::greedy::build_greedy_solution,
    evaluator::{Evaluator, TFTEvaluator},
    problem::Problem,
    solution::Solution,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    define_algorithm,
    tui::adapters::adapter::{Adapter, RunLog, RunnableAdapter, Settings},
};

define_algorithm!(AdapterGreedy, "Greedy Algorithm", "GR");

impl Default for AdapterGreedy {
    fn default() -> Self {
        Self {
            settings: String::new(),
        }
    }
}

impl RunnableAdapter for AdapterGreedy {
    fn run(&self, problem: &Problem, _initial: Option<&Solution>, tx: UnboundedSender<RunLog>) {
        let mut evaluator = TFTEvaluator::new(problem.processing_times.clone());
        let greedy_solution = build_greedy_solution(problem.jobs_number, &mut evaluator);
        let fitness = evaluator.evaluate(&greedy_solution);
        let _ = tx.send(RunLog {
            best: greedy_solution,
            fitness,
            message: format!(
                "built greedy solution, FFE count: {}",
                evaluator.eval_count()
            ),
        });
    }
}
