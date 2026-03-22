use std::io;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
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
    model::{event::AppEvent, model::AppModel},
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

    // TODO: print logs to stdout
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
        if let Some(event) = event_rx.recv().await {
            model.update_on_event(event);
        }
        while let Ok(event) = event_rx.try_recv() {
            model.update_on_event(event);
        }
    }

    token.cancel();
    Ok(())
}

async fn handle_keyboard_events(tx: UnboundedSender<AppEvent>, token: CancellationToken) {
    loop {
        tokio::select! {
            _ = token.cancelled() => break,
            maybe_event = tokio::task::spawn_blocking(|| AppEvent::read().ok().flatten()) => {
                if let Ok(Some(event)) = maybe_event {
                    if tx.send(event).is_err() {
                        break;
                    }
                }
            }
        }
    }
}
