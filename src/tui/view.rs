use pfsp_solver::solver::solution::Solution;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, List, ListItem, Paragraph},
};
use tui_textarea::TextArea;

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
    let items: Vec<ListItem> = model
        .algorithms
        .iter()
        .enumerate()
        .map(|(i, adapter)| {
            let prefix = if i == model.selected_algorithm {
                "+"
            } else {
                " "
            };
            let style = if i == model.selected_algorithm {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(format!("{} {}. {}", prefix, i + 1, adapter.name())).style(style)
        })
        .collect();
    let list = List::new(items).block(
        Block::default()
            .title("Algorithms")
            .borders(Borders::ALL)
            .border_style(style_for_screen(AppScreen::Algorithms, model)),
    );
    frame.render_widget(list, rect);
}

fn render_control_panel(frame: &mut Frame, model: &AppModel, rect: Rect) {
    let label = if model.algorithm_running {
        "Stop"
    } else {
        "Start"
    };
    let block = Block::default()
        .title("Run algorithm")
        .borders(Borders::ALL)
        .border_style(style_for_screen(AppScreen::ControlPanel, model));
    let paragraph = Paragraph::new(label).block(block);
    frame.render_widget(paragraph, rect);
}

fn render_main_panel(frame: &mut Frame, model: &AppModel, rect: Rect) {
    let block = Block::default().borders(Borders::ALL);
    frame.render_widget(&block, rect);
    let inner_rect = block.inner(rect);
    match model.screen {
        AppScreen::ProblemInstance => {
            render_main_panel_for_problem_description(frame, model, inner_rect)
        }
        AppScreen::CurrentSolution => {
            render_main_panel_for_solution_input(frame, model, inner_rect)
        }
        AppScreen::Algorithms => render_main_panel_for_algorithms(frame, model, inner_rect),
        AppScreen::ControlPanel => render_main_panel_for_control_panel(frame, model, inner_rect),
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

fn render_textarea(frame: &mut Frame, textarea: &TextArea, rect: Rect, is_focused: bool) {
    let (cursor_row, cursor_col) = textarea.cursor();
    let lines: Vec<Line> = textarea
        .lines()
        .iter()
        .enumerate()
        .map(|(i, line)| {
            if is_focused && i == cursor_row {
                let char_indices: Vec<usize> = line.char_indices().map(|(idx, _)| idx).collect();
                let byte_start = char_indices.get(cursor_col).copied().unwrap_or(line.len());
                let byte_end = char_indices
                    .get(cursor_col + 1)
                    .copied()
                    .unwrap_or(line.len());
                let before = &line[..byte_start];
                let cursor_char = if byte_start < line.len() {
                    &line[byte_start..byte_end]
                } else {
                    " "
                };
                let after = &line[byte_end..];
                Line::from(vec![
                    Span::raw(before.to_string()),
                    Span::styled(
                        cursor_char.to_string(),
                        Style::default().bg(Color::White).fg(Color::Black),
                    ),
                    Span::raw(after.to_string()),
                ])
            } else {
                Line::raw(line.to_string())
            }
        })
        .collect();
    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .title("Settings")
            .borders(Borders::ALL)
            .border_style(if is_focused {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            }),
    );
    frame.render_widget(paragraph, rect);
}

fn render_main_panel_for_algorithms(frame: &mut Frame, model: &AppModel, rect: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rect);

    let is_focused = model.is_focused && model.screen == AppScreen::Algorithms;
    render_textarea(frame, &model.settings_textarea, chunks[0], is_focused);

    let items: Vec<ListItem> = model
        .settings_textarea
        .lines()
        .iter()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, ':').map(|s| s.trim()).collect();
            if parts.len() == 2 && !parts[0].is_empty() {
                Some(ListItem::new(format!("{}: {}", parts[0], parts[1])))
            } else {
                None
            }
        })
        .collect();
    let parsed_list = List::new(items).block(
        Block::default()
            .title("Parsed Settings")
            .borders(Borders::ALL),
    );
    frame.render_widget(parsed_list, chunks[1]);
}

fn render_main_panel_for_control_panel(frame: &mut Frame, model: &AppModel, rect: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(50),
            Constraint::Min(0),
        ])
        .split(rect);

    let header = Paragraph::new("Enter: Start/Stop | r: Reset logs | j/k: Scroll")
        .block(Block::default().title("Controls").borders(Borders::ALL));
    frame.render_widget(header, chunks[0]);

    render_logs_list(frame, model, chunks[1]);
    render_fitness_chart(frame, model, chunks[2]);
}

fn render_logs_list(frame: &mut Frame, model: &AppModel, rect: Rect) {
    let num_width = model.run_logs.len().max(1).to_string().len();
    let inner_width = rect.width.saturating_sub(2) as usize;
    let visible_height = rect.height.saturating_sub(2) as usize;
    let fitness_col_width = 12;
    let message_col_width = inner_width.saturating_sub(fitness_col_width + num_width + 5);

    let scroll = if model.log_autoscroll {
        model.run_logs.len().saturating_sub(visible_height)
    } else {
        model
            .log_scroll
            .min(model.run_logs.len().saturating_sub(visible_height))
    };

    let items: Vec<ListItem> = model
        .run_logs
        .iter()
        .enumerate()
        .skip(scroll)
        .map(|(i, log)| {
            let message: String = log.message.chars().take(message_col_width).collect();
            ListItem::new(format!(
                "{:>nw$} {:<mw$} | {}",
                i + 1,
                message,
                log.fitness,
                nw = num_width,
                mw = message_col_width
            ))
        })
        .collect();
    let list = List::new(items).block(
        Block::default()
            .title(format!("Logs ({} entries)", model.run_logs.len()))
            .borders(Borders::ALL),
    );
    frame.render_widget(list, rect);
}

fn render_fitness_chart(frame: &mut Frame, model: &AppModel, rect: Rect) {
    if model.fitness_data.is_empty() {
        let empty = Paragraph::new("No data yet")
            .block(Block::default().title("Fitness").borders(Borders::ALL));
        frame.render_widget(empty, rect);
        return;
    }

    let max_x = model.fitness_data.len() as f64;
    let min_y = model
        .fitness_data
        .iter()
        .map(|p| p.1)
        .fold(f64::INFINITY, f64::min);
    let max_y = model
        .fitness_data
        .iter()
        .map(|p| p.1)
        .fold(f64::NEG_INFINITY, f64::max);
    let y_margin = if (max_y - min_y).abs() < f64::EPSILON {
        1.0
    } else {
        (max_y - min_y) * 0.1
    };

    let dataset = Dataset::default()
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .data(&model.fitness_data);

    let chart = Chart::new(vec![dataset])
        .block(Block::default().title("Fitness").borders(Borders::ALL))
        .x_axis(
            Axis::default()
                .title("Iteration")
                .bounds([0.0, max_x])
                .labels::<Vec<Line>>(vec!["0".into(), format!("{}", max_x as u64).into()]),
        )
        .y_axis(
            Axis::default()
                .title("Fitness")
                .bounds([min_y - y_margin, max_y + y_margin])
                .labels::<Vec<Line>>(vec![
                    format!("{}", min_y as u64).into(),
                    format!("{}", max_y as u64).into(),
                ]),
        );
    frame.render_widget(chart, rect);
}
