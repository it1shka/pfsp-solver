use std::cmp::max;

use crate::solver::problem::Time;

pub struct Solution(pub Vec<usize>);

impl Solution {
    pub fn calc_total_flow_time(&self, processing_times: &[Vec<Time>]) -> Time {
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
}
