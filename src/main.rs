use std::io::{self, Stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

/// Game state — everything we need to know about the world right now.
struct Game {
    /// Player X position (column in the game field)
    player_x: u16,
    /// Player Y position (row in the game field, 0 = bottom)
    player_y: u16,
    /// Current score
    score: u32,
}

fn main() -> io::Result<()> {
    // --- Terminal setup ---
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // --- Run game ---
    let result = run_game(&mut terminal);

    // --- Cleanup ---
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_game(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    // Initial game state
    let mut game = Game {
        player_x: 10,
        player_y: 0,
        score: 0,
    };

    loop {
        terminal.draw(|frame| draw(frame, &game))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }
    Ok(())
}

/// Draw one frame.
fn draw(frame: &mut Frame, game: &Game) {
    let size = frame.area();

    // Split screen: 3 rows at top for score, rest for game field
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // score bar
            Constraint::Min(0),     // game field
        ])
        .split(size);

    let score_area = chunks[0];
    let game_area = chunks[1];

    draw_score(frame, score_area, game);
    draw_game_field(frame, game_area, game);
}

/// Top bar: score display.
fn draw_score(frame: &mut Frame, area: Rect, game: &Game) {
    let text = Line::from(vec![
        Span::styled(" SCORE: ", Style::default().fg(Color::Yellow)),
        Span::styled(
            format!("{}", game.score),
            Style::default().fg(Color::White),
        ),
        Span::raw("   "),
        Span::styled("[q] quit", Style::default().fg(Color::DarkGray)),
    ]);

    let widget = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" DANGER DASH "),
    );

    frame.render_widget(widget, area);
}

/// The main play area: ground + player.
fn draw_game_field(frame: &mut Frame, area: Rect, game: &Game) {
    // First, draw the border/background
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    frame.render_widget(block, area);

    // Inner area (inside the border)
    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

    // Ground row: bottom line of inner area
    let ground_y = inner.y + inner.height - 1;

    // Draw ground: a horizontal line of '=' characters
    let ground_line: String = "=".repeat(inner.width as usize);
    let ground = Paragraph::new(ground_line).style(Style::default().fg(Color::Green));
    frame.render_widget(
        ground,
        Rect {
            x: inner.x,
            y: ground_y,
            width: inner.width,
            height: 1,
        },
    );

    // Draw player: '@' sitting on the ground
    // player_y = 0 means "on the ground", higher = floating above
    let player_screen_y = ground_y.saturating_sub(1 + game.player_y);
    let player_screen_x = inner.x + game.player_x;

    // Clamp so player no comot screen
    if player_screen_x < inner.x + inner.width && player_screen_y >= inner.y {
        let player = Paragraph::new("@").style(
            Style::default()
                .fg(Color::LightRed)
                .add_modifier(ratatui::style::Modifier::BOLD),
        );
        frame.render_widget(
            player,
            Rect {
                x: player_screen_x,
                y: player_screen_y,
                width: 1,
                height: 1,
            },
        );
    }
}

