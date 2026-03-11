use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub struct InputState {
    pub value: String,
    pub cursor: usize,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            value: String::new(),
            cursor: 0,
        }
    }

    pub fn cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn cursor_right(&mut self) {
        if self.cursor < self.value.len() {
            self.cursor += 1;
        }
    }

    pub fn add_symbol(&mut self, symbol: char) {
        self.value.insert(self.cursor, symbol);
        self.cursor += 1;
    }

    pub fn remove_symbol(&mut self) {
        if self.cursor > 0 {
            self.value.remove(self.cursor - 1);
            self.cursor -= 1;
        }
    }
}

pub fn render_input(title: &str, focus: bool, frame: &mut Frame, state: &InputState, rect: Rect) {
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(if focus {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    let inner_width = rect.width.saturating_sub(2) as usize;

    let scroll_offset = if inner_width == 0 {
        0
    } else if state.cursor >= inner_width {
        state.cursor - inner_width + 1
    } else {
        0
    };

    let end = (scroll_offset + inner_width).min(state.value.len());
    let before_cursor = &state.value[scroll_offset..state.cursor];
    let after_cursor = &state.value[state.cursor..end];
    let visible = format!("{}|{}", before_cursor, after_cursor);

    let input = Paragraph::new(visible).block(block);

    frame.render_widget(input, rect);
}
