use std::{cmp::max, collections::HashSet};

use rand::{Rng, seq::SliceRandom};

use crate::solver::problem::Time;

pub struct Solution {
    pub data: Vec<usize>,
}

impl Clone for Solution {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl Solution {
    pub fn empty() -> Self {
        Solution { data: vec![] }
    }

    pub fn new(value: Vec<usize>) -> Self {
        Solution { data: value }
    }

    pub fn parse(raw_solution: &str) -> Option<Self> {
        raw_solution
            .split_whitespace()
            .map(|chunk| str::parse::<usize>(chunk))
            .collect::<Result<Vec<_>, _>>()
            .ok()
            .map(|result| Solution { data: result })
    }

    pub fn random<R: Rng>(rng: &mut R, jobs_amount: usize) -> Self {
        let mut solution = (0..jobs_amount).collect::<Vec<usize>>();
        solution.shuffle(rng);
        Solution { data: solution }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_loosely_valid(&self, jobs_amount: usize) -> bool {
        let mut set = HashSet::new();
        for nth in &self.data {
            if *nth >= jobs_amount || !set.insert(*nth) {
                return false;
            }
        }
        true
    }

    pub fn is_valid(&self, jobs_amount: usize) -> bool {
        if self.data.len() != jobs_amount {
            return false;
        }
        self.is_loosely_valid(jobs_amount)
    }

    pub fn total_flow_time(&self, processing_times: &[Vec<Time>]) -> Time {
        let machines_number = processing_times.len();
        let mut clock = vec![0; machines_number];
        let mut total_time: Time = 0;
        for nth_job in &self.data {
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
        for nth_job in &self.data {
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
