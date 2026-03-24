use std::{io, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use pfsp_solver::solver::problem::Problem;
use ratatui::{
    Terminal,
    prelude::{Backend, CrosstermBackend},
};
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio_util::sync::CancellationToken;

use crate::tui::{
    model::{event::AppEvent, model::AppModel, screen::AppScreen},
    view::render_frame,
};

pub async fn start_application(problem: &Problem) -> color_eyre::Result<()> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut model = AppModel::new(problem);
    start_controller(&mut terminal, &mut model).await?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    Ok(())
}

async fn start_controller<'a, B: Backend>(
    terminal: &mut Terminal<B>,
    model: &mut AppModel<'a>,
) -> color_eyre::Result<()>
where
    color_eyre::Report: From<B::Error>,
{
    let (event_tx, mut event_rx) = mpsc::unbounded_channel();
    let token = CancellationToken::new();

    let keyboard_tx = event_tx.clone();
    let keyboard_token = token.clone();
    tokio::spawn(handle_keyboard_events(keyboard_tx, keyboard_token));

    while model.is_running {
        terminal.draw(|f| render_frame(f, model))?;
        if let Some(raw_event) = event_rx.recv().await {
            handle_event(model, &raw_event);
        }
        while let Ok(raw_event) = event_rx.try_recv() {
            handle_event(model, &raw_event);
        }
    }

    token.cancel();
    Ok(())
}

fn crossterm_to_textarea_input(key: &crossterm::event::KeyEvent) -> tui_textarea::Input {
    use tui_textarea::Key;
    let textarea_key = match key.code {
        KeyCode::Char(c) => Key::Char(c),
        KeyCode::Backspace => Key::Backspace,
        KeyCode::Enter => Key::Enter,
        KeyCode::Left => Key::Left,
        KeyCode::Right => Key::Right,
        KeyCode::Up => Key::Up,
        KeyCode::Down => Key::Down,
        KeyCode::Tab => Key::Tab,
        KeyCode::Delete => Key::Delete,
        KeyCode::Home => Key::Home,
        KeyCode::End => Key::End,
        KeyCode::PageUp => Key::PageUp,
        KeyCode::PageDown => Key::PageDown,
        KeyCode::Esc => Key::Esc,
        KeyCode::F(n) => Key::F(n),
        _ => Key::Null,
    };
    tui_textarea::Input {
        key: textarea_key,
        ctrl: key.modifiers.contains(KeyModifiers::CONTROL),
        alt: key.modifiers.contains(KeyModifiers::ALT),
        shift: key.modifiers.contains(KeyModifiers::SHIFT),
    }
}

fn handle_event(model: &mut AppModel, raw_event: &Event) {
    if model.is_focused && model.screen == AppScreen::Algorithms {
        if let Some(AppEvent::Escape) = AppEvent::from_crossterm(raw_event) {
            model.update_on_event(AppEvent::Escape);
        } else if let Event::Key(key) = raw_event {
            model
                .settings_textarea
                .input(crossterm_to_textarea_input(key));
        }
        return;
    }
    if let Some(app_event) = AppEvent::from_crossterm(raw_event) {
        model.update_on_event(app_event);
    }
}

const EVENT_POLL_INTERVAL_MS: u64 = 33;

async fn handle_keyboard_events(tx: UnboundedSender<Event>, token: CancellationToken) {
    loop {
        tokio::select! {
                    _ = token.cancelled() => break,
                    maybe_event = tokio::task::spawn_blocking(|| {
        event::poll(Duration::from_millis(EVENT_POLL_INTERVAL_MS))
                .unwrap_or(false)
                .then(|| event::read().ok()).flatten()
                    }) => {
                        if let Ok(Some(raw_event)) = maybe_event {
                            if matches!(raw_event, Event::Key(_))
                                && tx.send(raw_event).is_err() {
                                    break;
                                }
                        }
                    }
                }
    }
}
