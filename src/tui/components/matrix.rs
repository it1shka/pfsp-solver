use pfsp_solver::solver::problem::Time;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

pub struct MatrixState {
    pub row_offset: usize,
    pub col_offset: usize,
}

impl MatrixState {
    pub fn new() -> Self {
        MatrixState {
            row_offset: 0,
            col_offset: 0,
        }
    }

    pub fn move_left(&mut self) {
        if self.col_offset > 0 {
            self.col_offset -= 1;
        }
    }

    pub fn move_right(&mut self, max_cols: usize) {
        if self.col_offset + 1 < max_cols {
            self.col_offset += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.row_offset > 0 {
            self.row_offset -= 1;
        }
    }

    pub fn move_down(&mut self, max_rows: usize) {
        if self.row_offset + 1 < max_rows {
            self.row_offset += 1;
        }
    }
}

const COL_WIDTH: u16 = 7;
const ROW_LABEL_WIDTH: u16 = 6;

fn time_color(value: Time, min: Time, max: Time) -> Color {
    if min == max {
        return Color::Yellow;
    }
    let ratio = (value - min) as f64 / (max - min) as f64;
    if ratio < 0.33 {
        Color::Green
    } else if ratio < 0.66 {
        Color::Yellow
    } else {
        Color::Red
    }
}

pub fn render_matrix(frame: &mut Frame, rect: Rect, data: &[Vec<Time>], state: &MatrixState) {
    let block = Block::default()
        .title("Processing Times")
        .borders(Borders::ALL);

    if data.is_empty() {
        frame.render_widget(block, rect);
        return;
    }

    let min_val = data
        .iter()
        .flat_map(|r| r.iter())
        .copied()
        .min()
        .unwrap_or(0);
    let max_val = data
        .iter()
        .flat_map(|r| r.iter())
        .copied()
        .max()
        .unwrap_or(0);

    let inner_height = rect.height.saturating_sub(3) as usize;
    let inner_width = rect.width.saturating_sub(2 + ROW_LABEL_WIDTH);
    let visible_cols = (inner_width / COL_WIDTH) as usize;

    let num_rows = data.len();
    let num_cols = data.first().map_or(0, |r| r.len());

    let row_start = state.row_offset.min(num_rows.saturating_sub(1));
    let row_end = (row_start + inner_height).min(num_rows);

    let col_start = state.col_offset.min(num_cols.saturating_sub(1));
    let col_end = (col_start + visible_cols).min(num_cols);

    let header_cells = std::iter::once(Cell::from("")).chain((col_start..col_end).map(|j| {
        Cell::from(format!("J {}", j)).style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
    }));
    let header = Row::new(header_cells);

    let rows = (row_start..row_end).map(|i| {
        let label = Cell::from(format!("M {}", i)).style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        );
        let cells = std::iter::once(label).chain((col_start..col_end).map(|j| {
            Cell::from(data[i][j].to_string())
                .style(Style::default().fg(time_color(data[i][j], min_val, max_val)))
        }));
        Row::new(cells)
    });

    let widths: Vec<_> = std::iter::once(ratatui::layout::Constraint::Length(ROW_LABEL_WIDTH))
        .chain(
            std::iter::repeat(ratatui::layout::Constraint::Length(COL_WIDTH))
                .take(col_end - col_start),
        )
        .collect();

    let table = Table::new(rows, widths).header(header).block(block);

    frame.render_widget(table, rect);
}
