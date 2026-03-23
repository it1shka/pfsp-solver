use std::collections::HashMap;

use pfsp_solver::solver::{
    problem::{Problem, Time},
    solution::Solution,
};
use tokio::sync::mpsc::UnboundedSender;

pub type Settings = String;

pub trait Adapter {
    fn name(&self) -> &'static str;
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
    ($name:ident,$str_name:expr) => {
        pub struct $name {
            settings: Settings,
        }

        impl $name {
            fn new(settings: Settings) -> Self {
                Self { settings }
            }
        }

        impl Adapter for $name {
            fn name(&self) -> &'static str {
                $str_name
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
    async fn run(&self, problem: &Problem, initial: Option<&Solution>, tx: UnboundedSender<RunLog>);
}
