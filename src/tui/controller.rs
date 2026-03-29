use std::{io, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use pfsp_solver::solver::{problem::Problem, solution::Solution};
use ratatui::{
    Terminal,
    prelude::{Backend, CrosstermBackend},
};
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio_util::sync::CancellationToken;

use crate::tui::{
    adapters::{
        AdapterAnnealing, AdapterGA, AdapterGreedy, AdapterRandom,
        adapter::{Adapter, RunLog},
    },
    model::{event::AppEvent, model::AppModel, screen::AppScreen},
    view::render_frame,
};

enum ControllerEvent {
    Keyboard(Event),
    AlgorithmLog(RunLog),
    AlgorithmFinished,
}

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

    if !model.run_logs.is_empty() {
        println!("message,fitness,solution");
        for log in &model.run_logs {
            let solution_str: String = log
                .best
                .data
                .iter()
                .map(|j| j.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            println!(
                "\"{}\",{},\"{}\"",
                log.message.replace('"', "\"\""),
                log.fitness,
                solution_str
            );
        }
    }

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
        if let Some(controller_event) = event_rx.recv().await {
            process_controller_event(model, controller_event, &event_tx);
        }
        while let Ok(controller_event) = event_rx.try_recv() {
            process_controller_event(model, controller_event, &event_tx);
        }
    }

    if let Some(cancel_token) = model.cancellation_token.take() {
        cancel_token.cancel();
    }
    token.cancel();
    Ok(())
}

fn process_controller_event(
    model: &mut AppModel,
    event: ControllerEvent,
    event_tx: &UnboundedSender<ControllerEvent>,
) {
    match event {
        ControllerEvent::Keyboard(raw_event) => handle_event(model, &raw_event, event_tx),
        ControllerEvent::AlgorithmLog(log) => model.push_log(log),
        ControllerEvent::AlgorithmFinished => {
            model.algorithm_running = false;
            model.cancellation_token = None;
        }
    }
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

fn handle_event(
    model: &mut AppModel,
    raw_event: &Event,
    event_tx: &UnboundedSender<ControllerEvent>,
) {
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
        if model.is_focused
            && model.screen == AppScreen::ControlPanel
            && app_event == AppEvent::Enter
        {
            if model.algorithm_running {
                stop_algorithm(model);
            } else {
                start_algorithm(model, event_tx);
            }
            return;
        }
        model.update_on_event(app_event);
    }
}

fn build_adapter(index: usize, settings: String) -> Box<dyn crate::tui::adapters::RunnableAdapter> {
    match index {
        0 => Box::new(AdapterRandom::new(settings)),
        1 => Box::new(AdapterGreedy::new(settings)),
        2 => Box::new(AdapterAnnealing::new(settings)),
        3 => Box::new(AdapterGA::new(settings)),
        _ => unreachable!(),
    }
}

fn start_algorithm(model: &mut AppModel, event_tx: &UnboundedSender<ControllerEvent>) {
    if model.algorithm_running {
        return;
    }

    let old_content = model.settings_textarea.lines().join("\n");
    model.algorithms[model.selected_algorithm].set_settings(old_content);

    let adapter_index = model.selected_algorithm;
    let settings = model.algorithms[adapter_index].get_settings().clone();
    let short_name = model.algorithms[adapter_index].short_name().to_string();
    let problem = model.problem.clone();
    let initial =
        Solution::parse(&model.solution_input.value).filter(|s| s.is_valid(problem.jobs_number));

    let token = CancellationToken::new();
    model.cancellation_token = Some(token.clone());
    model.algorithm_running = true;

    let tx = event_tx.clone();
    tokio::spawn(async move {
        let (log_tx, mut log_rx) = mpsc::unbounded_channel::<RunLog>();
        let handle = tokio::task::spawn_blocking(move || {
            let adapter = build_adapter(adapter_index, settings);
            adapter.run(&problem, initial.as_ref(), log_tx);
        });
        tokio::select! {
            _ = token.cancelled() => {}
            _ = async {
                while let Some(mut log) = log_rx.recv().await {
                    log.message = format!("[{}] {}", short_name, log.message);
                    if tx.send(ControllerEvent::AlgorithmLog(log)).is_err() {
                        break;
                    }
                }
                handle.await.ok();
            } => {}
        }
        let _ = tx.send(ControllerEvent::AlgorithmFinished);
    });
}

fn stop_algorithm(model: &mut AppModel) {
    if let Some(token) = model.cancellation_token.take() {
        token.cancel();
    }
}

const EVENT_POLL_INTERVAL_MS: u64 = 33;

async fn handle_keyboard_events(tx: UnboundedSender<ControllerEvent>, token: CancellationToken) {
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
                                && tx.send(ControllerEvent::Keyboard(raw_event)).is_err() {
                                    break;
                                }
                        }
                    }
                }
    }
}
