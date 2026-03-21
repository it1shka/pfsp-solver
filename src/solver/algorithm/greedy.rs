use std::collections::HashSet;

use crate::solver::{evaluator::Evaluator, problem::Time, solution::Solution};

pub fn build_greedy_solution(jobs_amount: usize, evaluator: &mut dyn Evaluator) -> Solution {
    let mut solution = Solution::empty();
    let mut available_jobs: HashSet<usize> = HashSet::from_iter(0..jobs_amount);
    while !available_jobs.is_empty() {
        let (mut best_evaluation, mut best_job) = (Time::MAX, 0usize);
        for &job in &available_jobs {
            solution.push(job);
            let current_evaluation = evaluator.evaluate(&solution);
            solution.pop();
            if current_evaluation < best_evaluation {
                best_evaluation = current_evaluation;
                best_job = job;
            }
        }
        available_jobs.remove(&best_job);
        solution.push(best_job);
    }
    solution
}
