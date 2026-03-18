use std::io::{self, BufRead};

pub type Time = u64;

#[derive(Debug)]
pub struct Problem {
    pub jobs_number: usize,
    pub machines_number: usize,
    pub initial_seed: Option<u64>,
    pub upper_bound: Option<Time>,
    pub lower_bound: Option<Time>,
    pub processing_times: Vec<Vec<Time>>,
}

pub const MIN_JOBS_NUMBER: usize = 1;
pub const MIN_MACHINES_NUMBER: usize = 1;

impl Problem {
    pub fn parse<R: io::Read>(source: R) -> io::Result<Problem> {
        macro_rules! parser_err {
            ($msg:expr) => {
                io::Error::new(io::ErrorKind::InvalidData, $msg)
            };
        }

        let reader = io::BufReader::new(source);
        let mut lines = reader.lines();

        lines
            .next()
            .ok_or(parser_err!("Parameters header line was expected"))??;
        let raw_parameters = lines
            .next()
            .ok_or(parser_err!("Problem parameters were expected"))??;
        let parameters = raw_parameters.split_whitespace().collect::<Vec<_>>();
        if parameters.len() != 5 {
            return Err(parser_err!("Expected exactly 5 problem parameters"));
        }

        macro_rules! required_param {
            ($nth: expr, $type:ty, $err_msg:expr) => {
                parameters[$nth]
                    .parse::<$type>()
                    .map_err(|_| parser_err!($err_msg))?
            };
        }
        macro_rules! optional_param {
            ($nth: expr, $type:ty, $err_msg:expr) => {
                if parameters[$nth] == "-" {
                    None
                } else {
                    Some(required_param!($nth, $type, $err_msg))
                }
            };
        }

        let jobs_number = required_param!(
            0,
            usize,
            "Parameter for jobs number should be an unsigned integer"
        );
        if jobs_number < 1 {
            return Err(parser_err!(
                "Parameter for jobs number should be a positive number"
            ));
        }
        let machines_number = required_param!(
            1,
            usize,
            "Parameter for machines number should be an unsigned integer"
        );
        if machines_number < 1 {
            return Err(parser_err!(
                "Parameter for machines number should be a positive number"
            ));
        }
        let initial_seed = optional_param!(
            2,
            u64,
            "Parameter for initial seed should be an unsigned 64-bit integer or a hyphen"
        );
        let upper_bound = optional_param!(
            3,
            Time,
            "Parameter for upper bound should be an unsigned integer or a hyphen"
        );
        let lower_bound = optional_param!(
            4,
            Time,
            "Parameter for lower bound should be an unsigned integer or a hyphen"
        );

        lines
            .next()
            .ok_or(parser_err!("Processing times header line was expected"))??;
        let mut processing_times: Vec<Vec<Time>> = Vec::with_capacity(machines_number);
        for _ in 0..machines_number {
            let raw_machine_processing_times = lines
                .next()
                .ok_or(parser_err!("Processing times line was expected"))??;
            let machine_processing_times_or_err = raw_machine_processing_times
                .split_whitespace()
                .map(|processing_time| processing_time.parse::<Time>())
                .collect::<Result<Vec<_>, _>>();
            let machine_processing_times = machine_processing_times_or_err.map_err(|_| {
                parser_err!("Processing times line should contain only unsigned integers")
            })?;
            if machine_processing_times.len() != jobs_number {
                return Err(parser_err!(
                    "Processing times line has different width than the number of jobs"
                ));
            }
            processing_times.push(machine_processing_times)
        }
        Ok(Problem {
            jobs_number: jobs_number,
            machines_number: machines_number,
            initial_seed: initial_seed,
            upper_bound: upper_bound,
            lower_bound: lower_bound,
            processing_times: processing_times,
        })
    }

    pub fn is_valid(&self) -> bool {
        (self.jobs_number >= MIN_JOBS_NUMBER) && (self.machines_number >= MIN_MACHINES_NUMBER)
    }
}
