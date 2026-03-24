use std::collections::HashMap;

use pfsp_solver::solver::{
    problem::{Problem, Time},
    solution::Solution,
};
use tokio::sync::mpsc::UnboundedSender;

pub type Settings = String;

pub trait Adapter {
    fn new(settings: Settings) -> Self
    where
        Self: Sized;
    fn name(&self) -> &'static str;
    fn short_name(&self) -> &'static str;
    fn get_settings(&self) -> &Settings;
    fn set_settings(&mut self, new_settings: Settings);
    fn build_settings(&self) -> HashMap<String, String> {
        self.get_settings()
            .lines()
            .map(|line| {
                line.split(':')
                    .map(|chunk| chunk.trim())
                    .take(2)
                    .collect::<Vec<_>>()
            })
            .filter(|line| line.len() == 2)
            .map(|line| (String::from(line[0]), String::from(line[1])))
            .collect()
    }
}

#[macro_export]
macro_rules! define_algorithm {
    ($name:ident,$str_name:expr,$short:expr) => {
        pub struct $name {
            settings: Settings,
        }

        impl Adapter for $name {
            fn new(settings: Settings) -> Self {
                Self { settings }
            }
            fn name(&self) -> &'static str {
                $str_name
            }
            fn short_name(&self) -> &'static str {
                $short
            }
            fn get_settings(&self) -> &Settings {
                &self.settings
            }
            fn set_settings(&mut self, new_settings: Settings) {
                self.settings = new_settings;
            }
        }
    };
}

pub struct RunLog {
    pub message: String,
    pub fitness: Time,
    pub best: Solution,
}

pub trait RunnableAdapter: Adapter {
    fn run(&self, problem: &Problem, initial: Option<&Solution>, tx: UnboundedSender<RunLog>);
}
