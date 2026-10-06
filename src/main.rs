use std::io::{self, Stdout};
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod game;
mod screens;

use app::{App, Screen};
use screens::playing::FRAME_TIME;

fn main() -> io::Result<()> {
    // --- Terminal setup ---
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // --- Run app ---
    let result = run(&mut terminal);

    // --- Cleanup ---
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    let mut app = App::new();
    let mut last_tick = Instant::now();

    loop {
        // 1. Draw current frame
        terminal.draw(|frame| match &mut app.screen {
            Screen::Menu(menu) => screens::menu::draw(frame, menu),
            Screen::Playing(game) => screens::playing::draw(frame, game),
        })?;

        // 2. How long until the next physics tick?
        let timeout = FRAME_TIME
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::ZERO);

        // 3. Poll for input with that timeout. E go return true if key ready.
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // Linux sends both press and release — we only want press
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                // Force quit on Ctrl+C
                if key.code == KeyCode::Char('c')
                    && key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    break;
                }

                app.handle_key(key.code);

                if app.should_quit {
                    break;
                }
            }
        }

        // 4. Time to tick physics?
        if last_tick.elapsed() >= FRAME_TIME {
            if let Screen::Playing(game) = &mut app.screen {
                screens::playing::tick(game);
            }
            last_tick = Instant::now();
        }
    }

    Ok(())
}
