use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Frame, Terminal,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Backend,
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::state::{AppEvent, AppState};

const EVENT_POLL_TIME: u64 = 33;

pub fn render_loop<B: Backend>(
    terminal: &mut Terminal<B>,
    state: &mut AppState,
) -> color_eyre::Result<()>
where
    color_eyre::Report: From<B::Error>,
{
    while state.is_running {
        terminal.draw(|frame| {
            render_frame(frame, state);
        })?;
        if event::poll(Duration::from_millis(EVENT_POLL_TIME))? {
            let maybe_event = read_app_event()?;
            if let Some(event) = maybe_event {
                state.update_on_event(event);
            }
        }
    }
    Ok(())
}

fn read_app_event() -> io::Result<Option<AppEvent>> {
    let app_event = match event::read()? {
        Event::Key(key) => match key.code {
            KeyCode::Esc => Some(AppEvent::Close),
            KeyCode::Backspace => Some(AppEvent::DeleteSymbol),
            _ => key.code.as_char().map(|symbol| AppEvent::AddSymbol(symbol)),
        },
        _ => None,
    };
    Ok(app_event)
}

fn render_frame(frame: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(0)])
        .split(frame.area());
    render_sidebar(frame, state, chunks[0]);
    render_main_panel(frame, state, chunks[1]);
}

fn render_sidebar(frame: &mut Frame, state: &AppState, rect: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(rect);
    render_problem_description(frame, state, chunks[0]);
    render_solution_input(frame, state, chunks[1]);
}

fn render_problem_description(frame: &mut Frame, state: &AppState, rect: Rect) {
    let block = Block::default()
        .title("Problem Instance")
        .borders(Borders::ALL);
    frame.render_widget(&block, rect);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(block.inner(rect));
    let jobs_number_p = Paragraph::new(format!("Jobs: {}", state.problem.jobs_number));
    frame.render_widget(jobs_number_p, chunks[0]);
    let machines_number_p = Paragraph::new(format!("Machines: {}", state.problem.machines_number));
    frame.render_widget(machines_number_p, chunks[1]);
    let initial_seed_p = Paragraph::new(
        state
            .problem
            .initial_seed
            .map(|seed| format!("Seed: {}", seed))
            .unwrap_or(String::from("Seed: -")),
    );
    frame.render_widget(initial_seed_p, chunks[2]);
    let upper_bound_p = Paragraph::new(
        state
            .problem
            .upper_bound
            .map(|upper_bound| format!("Upper bound: {}", upper_bound))
            .unwrap_or(String::from("Upper bound: -")),
    );
    frame.render_widget(upper_bound_p, chunks[3]);
    let lower_bound_p = Paragraph::new(
        state
            .problem
            .upper_bound
            .map(|lower_bound| format!("Lower bound: {}", lower_bound))
            .unwrap_or(String::from("Lower bound: -")),
    );
    frame.render_widget(lower_bound_p, chunks[4]);
}

fn render_solution_input(frame: &mut Frame, state: &AppState, rect: Rect) {
    let input_p = Paragraph::new(state.raw_solution.clone()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Current Solution"),
    );
    frame.render_widget(input_p, rect);
}

fn render_main_panel(frame: &mut Frame, state: &AppState, rect: Rect) {
    // TODO:
}
