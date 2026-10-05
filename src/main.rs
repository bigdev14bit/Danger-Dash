use std::io::{self, Stdout};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod game;
mod screens;

use app::{App, Screen};

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

    loop {
        terminal.draw(|frame| match &app.screen {
            Screen::Menu(menu) => screens::menu::draw(frame, menu),
            Screen::Playing(game) => screens::playing::draw(frame, game),
        })?;

        if let Event::Key(key) = event::read()? {
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
    Ok(())
}

