use crossterm::event::{Event, KeyCode};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AppEvent {
    Escape,
    Enter,
    Backspace,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Key(char),
}

impl AppEvent {
    pub fn from_crossterm(event: &Event) -> Option<Self> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Esc => Some(Self::Escape),
                KeyCode::Enter => Some(Self::Enter),
                KeyCode::Backspace => Some(Self::Backspace),
                KeyCode::Up => Some(Self::ArrowUp),
                KeyCode::Down => Some(Self::ArrowDown),
                KeyCode::Left => Some(Self::ArrowLeft),
                KeyCode::Right => Some(Self::ArrowRight),
                _ => key.code.as_char().map(Self::Key),
            },
            _ => None,
        }
    }
}
