use std::io;

use crossterm::event::{self, Event, KeyCode};

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
    pub fn read() -> io::Result<Option<Self>> {
        let app_event = match event::read()? {
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
        };
        Ok(app_event)
    }
}
