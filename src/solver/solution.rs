use std::{cmp::max, collections::HashSet};

use crate::solver::problem::Time;

pub struct Solution(pub Vec<usize>);

impl Solution {
    pub fn empty() -> Self {
        Solution(vec![])
    }

    pub fn parse(raw_solution: &str) -> Option<Solution> {
        raw_solution
            .split_whitespace()
            .map(|chunk| str::parse::<usize>(chunk))
            .collect::<Result<Vec<_>, _>>()
            .ok()
            .map(|result| Solution(result))
    }

    pub fn is_loosely_valid(&self, jobs_amount: usize) -> bool {
        let mut set = HashSet::new();
        for nth in &self.0 {
            if *nth >= jobs_amount || !set.insert(*nth) {
                return false;
            }
        }
        true
    }

    pub fn is_valid(&self, jobs_amount: usize) -> bool {
        if self.0.len() != jobs_amount {
            return false;
        }
        self.is_loosely_valid(jobs_amount)
    }

    pub fn total_flow_time(&self, processing_times: &[Vec<Time>]) -> Time {
        let machines_number = processing_times.len();
        let mut clock = vec![0; machines_number];
        let mut total_time: Time = 0;
        for nth_job in &self.0 {
            let mut running_clock = clock[0];
            for nth_machine in 0..machines_number {
                running_clock = max(clock[nth_machine], running_clock)
                    + processing_times[nth_machine][*nth_job];
                clock[nth_machine] = running_clock;
            }
            total_time += running_clock;
        }
        total_time
    }

    pub fn graph_data(&self, processing_times: &[Vec<Time>]) -> Vec<Vec<(Time, Time)>> {
        let machines_number = processing_times.len();
        let mut clock = vec![0; machines_number];
        let mut data = vec![vec![]; machines_number];
        for nth_job in &self.0 {
            let mut running_clock = clock[0];
            for nth_machine in 0..machines_number {
                let time_start = max(clock[nth_machine], running_clock);
                running_clock = time_start + processing_times[nth_machine][*nth_job];
                data[nth_machine].push((time_start, running_clock));
                clock[nth_machine] = running_clock;
            }
        }
        data
    }
}
