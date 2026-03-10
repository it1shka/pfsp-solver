use pfsp_solver::solver::problem::Problem;

pub struct AppState<'a> {
    pub problem: &'a Problem,
    pub is_running: bool,
    pub raw_solution: String,
}

#[derive(Clone, Copy)]
pub enum AppEvent {
    Close,
    DeleteSymbol,
    AddSymbol(char),
}

impl<'a> AppState<'a> {
    pub fn new(problem: &'a Problem) -> Self {
        AppState {
            problem: problem,
            is_running: true,
            raw_solution: String::new(),
        }
    }

    pub fn update_on_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::Close => self.is_running = false,
            AppEvent::DeleteSymbol => {
                self.raw_solution.pop();
            }
            AppEvent::AddSymbol(symbol) => {
                self.raw_solution.push(symbol);
            }
        }
    }
}
