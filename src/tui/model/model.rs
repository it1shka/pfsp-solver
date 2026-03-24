use pfsp_solver::solver::problem::Problem;

use crate::tui::{
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
}

impl<'a> AppModel<'a> {
    pub fn new(problem: &'a Problem) -> Self {
        Self {
            is_running: true,
            is_focused: true,
            problem,
            screen: AppScreen::ProblemInstance,
            solution_input: InputState::new(),
            processing_times_matrix: MatrixState::new(),
        }
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
            _ => {}
        }
    }
}
