use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Frame, Terminal,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Backend,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::tui::state::{AppEvent, AppScreen, AppState};

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
            KeyCode::Up => Some(AppEvent::PrevScreen),
            KeyCode::Down => Some(AppEvent::NextScreen),
            _ => key.code.as_char().map(|symbol| AppEvent::AddSymbol(symbol)),
        },
        _ => None,
    };
    Ok(app_event)
}

fn style_for_screen(screen: AppScreen, state: &AppState) -> Style {
    if screen == state.screen {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    }
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
            Constraint::Length(7),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(rect);
    render_problem_description(frame, state, chunks[0]);
    render_solution_input(frame, state, chunks[1]);
    render_algorithms(frame, state, chunks[2]);
    render_control_panel(frame, state, chunks[3]);
}

fn render_problem_description(frame: &mut Frame, state: &AppState, rect: Rect) {
    let items = [
        ListItem::new(format!("Jobs: {}", state.problem.jobs_number)),
        ListItem::new(format!("Machines: {}", state.problem.machines_number)),
        ListItem::new(
            state
                .problem
                .initial_seed
                .map(|seed| format!("Seed: {}", seed))
                .unwrap_or(String::from("Seed: -")),
        ),
        ListItem::new(
            state
                .problem
                .upper_bound
                .map(|upper_bound| format!("Upper bound: {}", upper_bound))
                .unwrap_or(String::from("Upper bound: -")),
        ),
        ListItem::new(
            state
                .problem
                .lower_bound
                .map(|lower_bound| format!("Lower bound: {}", lower_bound))
                .unwrap_or(String::from("Lower bound: -")),
        ),
    ];
    let list = List::new(items).block(
        Block::default()
            .title("Problem Instance")
            .borders(Borders::ALL)
            .border_style(style_for_screen(AppScreen::ProblemInstance, state)),
    );
    frame.render_widget(list, rect);
}

fn render_solution_input(frame: &mut Frame, state: &AppState, rect: Rect) {
    let mut input_value = state.raw_solution.clone();
    if state.screen == AppScreen::CurrentSolution {
        input_value.push('_');
    }
    let input_p = Paragraph::new(input_value).block(
        Block::default()
            .title("Current Solution")
            .borders(Borders::ALL)
            .border_style(style_for_screen(AppScreen::CurrentSolution, state)),
    );
    frame.render_widget(input_p, rect);
}

fn render_algorithms(frame: &mut Frame, state: &AppState, rect: Rect) {
    let block = Block::default()
        .title("Algorithms")
        .borders(Borders::ALL)
        .border_style(style_for_screen(AppScreen::Algorithms, state));
    // TODO:
    frame.render_widget(block, rect);
}

fn render_control_panel(frame: &mut Frame, state: &AppState, rect: Rect) {
    let block = Block::default()
        .title("Run algorithm")
        .borders(Borders::ALL)
        .border_style(style_for_screen(AppScreen::ControlPanel, state));
    // TODO:
    frame.render_widget(block, rect);
}

fn render_main_panel(frame: &mut Frame, state: &AppState, rect: Rect) {
    let block = Block::default().borders(Borders::ALL);
    // TODO:
    frame.render_widget(block, rect);
}
