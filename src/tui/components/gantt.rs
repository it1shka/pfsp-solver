use ratatui::{
    Frame,
    layout::Rect,
    style::Color,
    symbols::Marker,
    widgets::{
        Block, Borders,
        canvas::{Canvas, Line},
    },
};

pub fn render_gantt_chart(title: &str, frame: &mut Frame, area: Rect, data: &[Vec<(u64, u64)>]) {
    let max_time = data
        .iter()
        .flat_map(|m| m.iter().map(|&(_, end)| end))
        .max()
        .unwrap_or(1) as f64;

    let num_machines = data.len() as f64;

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(title))
        .marker(Marker::Braille)
        .x_bounds([0.0, max_time])
        .y_bounds([-1.0, num_machines])
        .paint(|ctx| {
            for (m_idx, machine_jobs) in data.iter().enumerate() {
                let y_center = num_machines - m_idx as f64 - 1.0;

                for (j_idx, (start, end)) in machine_jobs.iter().enumerate() {
                    let color = Color::Indexed((j_idx % 7 + 1) as u8);
                    let x1 = *start as f64;
                    let x2 = *end as f64;

                    let mut current_y = y_center - 0.4;
                    while current_y <= y_center + 0.4 {
                        ctx.draw(&Line {
                            x1,
                            y1: current_y,
                            x2,
                            y2: current_y,
                            color,
                        });
                        current_y += 0.05;
                    }
                }
            }
        });

    frame.render_widget(canvas, area);
}
