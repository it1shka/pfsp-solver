use pfsp_solver::solver::problem::Problem;
use tokio_util::sync::CancellationToken;
use tui_textarea::TextArea;

use crate::tui::{
    adapters::{
        AdapterAnnealing, AdapterGA, AdapterGreedy, AdapterRandom, RunnableAdapter, adapter::RunLog,
    },
    components::{input::InputState, matrix::MatrixState},
    model::{event::AppEvent, screen::AppScreen},
};

pub struct AppModel<'a> {
    pub is_running: bool,
    pub is_focused: bool,
    pub problem: &'a Problem,
    pub screen: AppScreen,
    pub solution_input: InputState,
    pub processing_times_matrix: MatrixState,
    pub algorithms: Vec<Box<dyn RunnableAdapter>>,
    pub selected_algorithm: usize,
    pub settings_textarea: TextArea<'static>,
    pub run_logs: Vec<RunLog>,
    pub fitness_data: Vec<(f64, f64)>,
    pub log_scroll: usize,
    pub log_autoscroll: bool,
    pub algorithm_running: bool,
    pub cancellation_token: Option<CancellationToken>,
}

fn build_textarea(content: &str) -> TextArea<'static> {
    let lines: Vec<String> = content.lines().map(String::from).collect();
    TextArea::new(lines)
}

impl<'a> AppModel<'a> {
    pub fn new(problem: &'a Problem) -> Self {
        let algorithms: Vec<Box<dyn RunnableAdapter>> = vec![
            Box::new(AdapterRandom::default()),
            Box::new(AdapterGreedy::default()),
            Box::new(AdapterAnnealing::default()),
            Box::new(AdapterGA::default()),
        ];
        let settings_textarea = build_textarea(algorithms[0].get_settings());
        Self {
            is_running: true,
            is_focused: true,
            problem,
            screen: AppScreen::ProblemInstance,
            solution_input: InputState::new(),
            processing_times_matrix: MatrixState::new(),
            algorithms,
            selected_algorithm: 0,
            settings_textarea,
            run_logs: Vec::new(),
            fitness_data: Vec::new(),
            log_scroll: 0,
            log_autoscroll: true,
            algorithm_running: false,
            cancellation_token: None,
        }
    }

    pub fn switch_algorithm(&mut self, new_index: usize) {
        if new_index >= self.algorithms.len() || new_index == self.selected_algorithm {
            return;
        }
        let old_content = self.settings_textarea.lines().join("\n");
        self.algorithms[self.selected_algorithm].set_settings(old_content);
        self.selected_algorithm = new_index;
        self.settings_textarea = build_textarea(self.algorithms[new_index].get_settings());
    }

    pub fn push_log(&mut self, log: RunLog) {
        self.fitness_data
            .push((self.fitness_data.len() as f64, log.fitness as f64));
        let best_str = log
            .best
            .data
            .iter()
            .map(|j| j.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        self.solution_input.value = best_str;
        self.run_logs.push(log);
    }

    pub fn reset_logs(&mut self) {
        self.run_logs.clear();
        self.fitness_data.clear();
        self.log_scroll = 0;
        self.log_autoscroll = true;
    }

    pub fn update_on_event(&mut self, event: AppEvent) {
        use AppEvent::*;
        use AppScreen::*;

        if self.is_focused {
            if event == Escape {
                self.is_focused = false;
                return;
            }
            match self.screen {
                ProblemInstance => match event {
                    ArrowUp | Key('k') => self.processing_times_matrix.move_up(),
                    ArrowDown | Key('j') => self
                        .processing_times_matrix
                        .move_down(self.problem.jobs_number),
                    ArrowLeft | Key('h') => self.processing_times_matrix.move_left(),
                    ArrowRight | Key('l') => self
                        .processing_times_matrix
                        .move_right(self.problem.jobs_number),
                    _ => {}
                },
                CurrentSolution => match event {
                    Backspace => self.solution_input.remove_symbol(),
                    ArrowLeft => self.solution_input.cursor_left(),
                    ArrowRight => self.solution_input.cursor_right(),
                    Key(symbol) => self.solution_input.add_symbol(symbol),
                    _ => {}
                },
                Algorithms => {}
                ControlPanel => match event {
                    Backspace => self.solution_input.clear(),
                    Key('r') => self.reset_logs(),
                    ArrowUp | Key('k') => {
                        self.log_autoscroll = false;
                        self.log_scroll = self.log_scroll.saturating_sub(1);
                    }
                    ArrowDown | Key('j') => {
                        self.log_scroll += 1;
                    }
                    _ => {}
                },
            }
            return;
        }
        match event {
            Escape => {
                self.is_running = false;
            }
            Enter => self.is_focused = true,
            ArrowUp | Key('k') => {
                self.screen = self.screen.prev_screen();
            }
            ArrowDown | Key('j') => {
                self.screen = self.screen.next_screen();
            }
            Backspace if self.screen == CurrentSolution => {
                self.solution_input.clear();
            }
            Key(ch) if self.screen == Algorithms && !self.algorithm_running => {
                if let Some(idx) = ch.to_digit(10) {
                    let idx = idx as usize;
                    if idx >= 1 && idx <= self.algorithms.len() {
                        self.switch_algorithm(idx - 1);
                    }
                }
            }
            _ => {}
        }
    }
}
