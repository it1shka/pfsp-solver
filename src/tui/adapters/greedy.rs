use pfsp_solver::solver::{problem::Problem, solution::Solution};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    define_algorithm,
    tui::adapters::adapter::{Adapter, RunLog, RunnableAdapter, Settings},
};

define_algorithm!(AdapterGreedy, "Greedy Algorithm");

impl Default for AdapterGreedy {
    fn default() -> Self {
        Self {
            settings: String::new(),
        }
    }
}

impl RunnableAdapter for AdapterGreedy {
    async fn run(
        &self,
        problem: &Problem,
        initial: Option<&Solution>,
        tx: UnboundedSender<RunLog>,
    ) {
    }
}
