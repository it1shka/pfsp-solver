use pfsp_solver::solver::problem::Problem;

pub struct AppState<'a> {
    pub problem: &'a Problem,
    pub is_running: bool,
    pub screen: AppScreen,
    pub raw_solution: String,
}

#[derive(Clone, Copy, PartialEq)]
pub enum AppScreen {
    ProblemInstance,
    CurrentSolution,
    Algorithms,
    ControlPanel,
}

impl AppScreen {
    fn next_screen(&self) -> Self {
        match self {
            AppScreen::ProblemInstance => AppScreen::CurrentSolution,
            AppScreen::CurrentSolution => AppScreen::Algorithms,
            AppScreen::Algorithms => AppScreen::ControlPanel,
            AppScreen::ControlPanel => AppScreen::ProblemInstance,
        }
    }

    fn prev_screen(&self) -> Self {
        match self {
            AppScreen::ProblemInstance => AppScreen::ControlPanel,
            AppScreen::CurrentSolution => AppScreen::ProblemInstance,
            AppScreen::Algorithms => AppScreen::CurrentSolution,
            AppScreen::ControlPanel => AppScreen::Algorithms,
        }
    }
}

#[derive(Clone, Copy)]
pub enum AppEvent {
    Close,
    PrevScreen,
    NextScreen,
    DeleteSymbol,
    AddSymbol(char),
}

impl<'a> AppState<'a> {
    pub fn new(problem: &'a Problem) -> Self {
        AppState {
            problem: problem,
            is_running: true,
            screen: AppScreen::ProblemInstance,
            raw_solution: String::new(),
        }
    }

    pub fn update_on_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::NextScreen => self.screen = self.screen.next_screen(),
            AppEvent::PrevScreen => self.screen = self.screen.prev_screen(),
            AppEvent::Close => self.is_running = false,
            _ => match self.screen {
                AppScreen::CurrentSolution => match event {
                    AppEvent::DeleteSymbol => {
                        self.raw_solution.pop();
                    }
                    AppEvent::AddSymbol(symbol) => {
                        self.raw_solution.push(symbol);
                    }
                    _ => {}
                },
                _ => {}
            },
        }
    }
}
