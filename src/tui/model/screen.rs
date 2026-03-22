#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    ProblemInstance,
    CurrentSolution,
    Algorithms,
    ControlPanel,
}

impl AppScreen {
    pub fn next_screen(&self) -> Self {
        use AppScreen::*;
        match self {
            ProblemInstance => CurrentSolution,
            CurrentSolution => Algorithms,
            Algorithms => ControlPanel,
            ControlPanel => ProblemInstance,
        }
    }

    pub fn prev_screen(&self) -> Self {
        use AppScreen::*;
        match self {
            ProblemInstance => ControlPanel,
            CurrentSolution => ProblemInstance,
            Algorithms => CurrentSolution,
            ControlPanel => Algorithms,
        }
    }
}
