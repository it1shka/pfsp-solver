use rand::{Rng, RngExt};

use crate::solver::{
    algorithm::operators::UnaryOperator, evaluator::Evaluator, helpers::select_roulette,
    problem::Time, solution::Solution,
};

#[derive(Clone, Copy)]
pub struct AnnealingStats {
    pub operator_used: &'static str,
    pub candidate_time: Time,
    pub delta: i128,
    pub accept_probability: f64,
    pub got_accepted: bool,
}

pub struct SimulatedAnnealing<R: Rng> {
    rng: R,
    pub temperature: f64,
    pub decay: f64,
    pub threshold: f64,
    pub solution: Solution,
    pub evaluator: Box<dyn Evaluator>,
    pub unary_ops: Vec<(Box<dyn UnaryOperator<R>>, f32)>,
}

impl<R: Rng> SimulatedAnnealing<R> {
    pub fn new(
        rng: R,
        solution: Solution,
        evaluator: Box<dyn Evaluator>,
        unary_ops: Vec<Box<dyn UnaryOperator<R>>>,
        temperature: f64,
        decay: f64,
        threshold: f64,
    ) -> Self {
        Self {
            rng,
            solution,
            evaluator,
            unary_ops: unary_ops
                .into_iter()
                .map(|op| {
                    let p = op.probability();
                    (op, p)
                })
                .collect(),
            temperature,
            decay,
            threshold,
        }
    }

    pub fn annealing_cycle(&mut self) -> AnnealingStats {
        let operator = select_roulette(&mut self.rng, &self.unary_ops);
        let mut candidate = self.solution.clone();
        operator.mutate(&mut self.rng, &mut candidate);

        let current_time = self.evaluator.evaluate(&self.solution);
        let candidate_time = self.evaluator.evaluate(&candidate);
        let delta = (candidate_time as i128) - (current_time as i128);
        let accept_probability = if delta < 0 {
            1.0
        } else {
            ((-delta as f64) / self.temperature).exp()
        };
        let got_accepted = self.rng.random_bool(accept_probability);
        if got_accepted {
            self.solution = candidate;
        }
        self.temperature *= self.decay;
        AnnealingStats {
            operator_used: operator.name(),
            candidate_time,
            delta,
            accept_probability,
            got_accepted,
        }
    }

    pub fn is_cold(&self) -> bool {
        self.temperature < self.threshold
    }
}
