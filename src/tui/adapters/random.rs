use pfsp_solver::solver::{problem::Problem, solution::Solution};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    define_algorithm,
    tui::adapters::adapter::{Adapter, RunLog, RunnableAdapter, Settings},
};

define_algorithm!(AdapterRandom, "Random Search");

impl Default for AdapterRandom {
    fn default() -> Self {
        Self {
            settings: String::new(),
        }
    }
}

impl RunnableAdapter for AdapterRandom {
    async fn run(
        &self,
        problem: &Problem,
        initial: Option<&Solution>,
        tx: UnboundedSender<RunLog>,
    ) {
    }
}
