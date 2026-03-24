use pfsp_solver::solver::problem::Problem;
use tui_textarea::TextArea;

use crate::tui::{
    adapters::{AdapterAnnealing, AdapterGA, AdapterGreedy, AdapterRandom, RunnableAdapter},
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
                ControlPanel => {}
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
            Key(ch) if self.screen == Algorithms => {
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
