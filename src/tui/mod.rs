use std::io;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use pfsp_solver::solver::problem::Problem;
use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::tui::{state::AppState, view::render_loop};

mod state;
mod view;

pub fn run_tui(problem: &Problem) -> color_eyre::Result<()> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState::new(problem);
    render_loop(&mut terminal, &mut state)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    // TODO: print logs to stdout
    Ok(())
}
