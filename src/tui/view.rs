use pfsp_solver::solver::solution::Solution;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::tui::{
    components::{gantt::render_gantt_chart, input::render_input, matrix::render_matrix},
    model::{model::AppModel, screen::AppScreen},
};

fn style_for_screen(screen: AppScreen, model: &AppModel) -> Style {
    if screen == model.screen {
        Style::default().fg(if model.is_focused {
            Color::Green
        } else {
            Color::Yellow
        })
    } else {
        Style::default()
    }
}

pub fn render_frame(frame: &mut Frame, model: &AppModel) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(0)])
        .split(frame.area());
    render_sidebar(frame, model, chunks[0]);
    render_main_panel(frame, model, chunks[1]);
}

fn render_sidebar(frame: &mut Frame, model: &AppModel, rect: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(rect);
    render_problem_description(frame, model, chunks[0]);
    render_solution_input(frame, model, chunks[1]);
    render_algorithms(frame, model, chunks[2]);
    render_control_panel(frame, model, chunks[3]);
}

fn render_problem_description(frame: &mut Frame, model: &AppModel, rect: Rect) {
    let items = [
        ListItem::new(format!("Jobs: {}", model.problem.jobs_number)),
        ListItem::new(format!("Machines: {}", model.problem.machines_number)),
        ListItem::new(
            model
                .problem
                .initial_seed
                .map(|seed| format!("Seed: {}", seed))
                .unwrap_or(String::from("Seed: -")),
        ),
        ListItem::new(
            model
                .problem
                .upper_bound
                .map(|upper_bound| format!("Upper bound: {}", upper_bound))
                .unwrap_or(String::from("Upper bound: -")),
        ),
        ListItem::new(
            model
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
            .border_style(style_for_screen(AppScreen::ProblemInstance, model)),
    );
    frame.render_widget(list, rect);
}

fn render_solution_input(frame: &mut Frame, model: &AppModel, rect: Rect) {
    render_input(
        "Current Solution",
        style_for_screen(AppScreen::CurrentSolution, model),
        frame,
        &model.solution_input,
        rect,
    );
}

fn render_algorithms(frame: &mut Frame, model: &AppModel, rect: Rect) {
    let block = Block::default()
        .title("Algorithms")
        .borders(Borders::ALL)
        .border_style(style_for_screen(AppScreen::Algorithms, model));
    // TODO:
    frame.render_widget(block, rect);
}

fn render_control_panel(frame: &mut Frame, model: &AppModel, rect: Rect) {
    let block = Block::default()
        .title("Run algorithm")
        .borders(Borders::ALL)
        .border_style(style_for_screen(AppScreen::ControlPanel, model));
    // TODO:
    frame.render_widget(block, rect);
}

fn render_main_panel(frame: &mut Frame, model: &AppModel, rect: Rect) {
    let block = Block::default().borders(Borders::ALL);
    // TODO:
    frame.render_widget(&block, rect);
    let inner_rect = block.inner(rect);
    match model.screen {
        AppScreen::ProblemInstance => {
            render_main_panel_for_problem_description(frame, model, inner_rect)
        }
        AppScreen::CurrentSolution => {
            render_main_panel_for_solution_input(frame, model, inner_rect)
        }
        _ => {} // TODO:
    };
}

fn render_main_panel_for_problem_description(frame: &mut Frame, model: &AppModel, rect: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(0)])
        .split(rect);

    let items = [
        ListItem::new(format!("Jobs: {}", model.problem.jobs_number)),
        ListItem::new(format!("Machines: {}", model.problem.machines_number)),
        ListItem::new(
            model
                .problem
                .initial_seed
                .map(|seed| format!("Seed: {}", seed))
                .unwrap_or(String::from("Seed: -")),
        ),
        ListItem::new(
            model
                .problem
                .upper_bound
                .map(|upper_bound| format!("Upper bound: {}", upper_bound))
                .unwrap_or(String::from("Upper bound: -")),
        ),
        ListItem::new(
            model
                .problem
                .lower_bound
                .map(|lower_bound| format!("Lower bound: {}", lower_bound))
                .unwrap_or(String::from("Lower bound: -")),
        ),
    ];
    let list = List::new(items).block(
        Block::default()
            .title("Problem Instance")
            .borders(Borders::ALL),
    );
    frame.render_widget(list, chunks[0]);

    render_matrix(
        frame,
        chunks[1],
        &model.problem.processing_times,
        &model.processing_times_matrix,
    );
}

fn render_main_panel_for_solution_input(frame: &mut Frame, model: &AppModel, rect: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(0)])
        .split(rect);

    render_input(
        "Current Solution",
        Style::default(),
        frame,
        &model.solution_input,
        chunks[0],
    );

    let parsed_solution = Solution::parse(&model.solution_input.value);

    if let Some(solution) = parsed_solution
        && solution.is_loosely_valid(model.problem.jobs_number)
    {
        let total_flow_time = solution.total_flow_time(&model.problem.processing_times);
        let graph_data = solution.graph_data(&model.problem.processing_times);

        let inner_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(chunks[1]);

        let total_flow_time_p = Paragraph::new(total_flow_time.to_string()).block(
            Block::default()
                .title("Total Flow Time")
                .borders(Borders::ALL),
        );
        frame.render_widget(total_flow_time_p, inner_chunks[0]);

        render_gantt_chart("Flow Visualization", frame, inner_chunks[1], &graph_data);

        return;
    }

    let error = Paragraph::new("The solution is invalid")
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    let vertical_padding = chunks[1].height.saturating_sub(3) / 2;
    let centered = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(vertical_padding),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(chunks[1]);
    frame.render_widget(error, centered[1]);
}
