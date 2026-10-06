use std::time::Duration;

use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::game::state::Game;

/// How many rows thick the platform is.
const PLATFORM_HEIGHT: u16 = 8;

/// Target frame time. 16ms ≈ 60 FPS. Change for slower/faster feel.
pub const FRAME_TIME: Duration = Duration::from_millis(50);

pub fn draw(frame: &mut Frame, game: &mut Game) {
    let size = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(size);

    draw_score(frame, chunks[0], game);
    draw_game_field(frame, chunks[1], game);
}

pub enum PlayingAction {
    Continue,
    Quit,
}

/// Handle a single key press. Called once per key event.
pub fn handle_key(game: &mut Game, key: KeyCode) -> PlayingAction {
    let max_x = game.field_width.saturating_sub(1);

    match key {
        KeyCode::Char('q') => PlayingAction::Quit,
        KeyCode::Left | KeyCode::Char('a') => {
            game.player.move_left();
            PlayingAction::Continue
        }
        KeyCode::Right | KeyCode::Char('d') => {
            game.player.move_right(max_x);
            PlayingAction::Continue
        }
        KeyCode::Char(' ') => {
            game.player.try_jump();
            PlayingAction::Continue
        }
        _ => PlayingAction::Continue,
    }
}

/// Advance physics and other time-based state by one frame.
pub fn tick(game: &mut Game) {
    // Ground is y = 0 (player's "on top of platform" position).
    game.player.tick(0);
}

fn draw_score(frame: &mut Frame, area: Rect, game: &Game) {
    let text = Line::from(vec![
        Span::styled(" SCORE: ", Style::default().fg(Color::Yellow)),
        Span::styled(format!("{}", game.score), Style::default().fg(Color::White)),
        Span::raw("   "),
        Span::styled(
            "[←/→] move  [space] jump  [q] quit",
            Style::default().fg(Color::DarkGray),
        ),
    ]);

    let widget = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" DANGER DASH "),
    );

    frame.render_widget(widget, area);
}

fn draw_game_field(frame: &mut Frame, area: Rect, game: &mut Game) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    game.field_width = inner.width;
    game.field_height = inner.height;

    draw_platform(frame, inner);

    // Platform top row (in screen coords)
    let platform_top_y = inner.y + inner.height.saturating_sub(PLATFORM_HEIGHT);

    // Player screen position.
    // player.y = 0 means "on the platform top".
    // Higher player.y means higher on screen (subtract from platform_top_y).
    let player_screen_y_signed = (platform_top_y as i32) - 1 - game.player.y;
    let player_screen_x = inner.x + game.player.x;

    // Only draw if within the playfield.
    if player_screen_x < inner.x + inner.width
        && player_screen_y_signed >= inner.y as i32
        && (player_screen_y_signed as u16) < platform_top_y
    {
        let player = Paragraph::new("@").style(
            Style::default()
                .fg(Color::LightRed)
                .add_modifier(Modifier::BOLD),
        );
        frame.render_widget(
            player,
            Rect {
                x: player_screen_x,
                y: player_screen_y_signed as u16,
                width: 1,
                height: 1,
            },
        );
    }
}

fn draw_platform(frame: &mut Frame, inner: Rect) {
    if inner.height < PLATFORM_HEIGHT {
        return;
    }

    let platform_top = inner.y + inner.height - PLATFORM_HEIGHT;
    let width = inner.width as usize;

    let top_row: String = "▀".repeat(width);
    let dirt_row: String = "█".repeat(width);
    let base_row: String = "▓".repeat(width);

    let top_color = Color::Rgb(120, 200, 80);
    let dirt_color = Color::Rgb(140, 90, 60);
    let base_color = Color::Rgb(80, 55, 40);

    // 8 rows: 1 grass, 4 dirt, 3 dark base
    let rows: [(u16, &String, Color); 8] = [
        (0, &top_row, top_color),
        (1, &dirt_row, dirt_color),
        (2, &dirt_row, dirt_color),
        (3, &dirt_row, dirt_color),
        (4, &dirt_row, dirt_color),
        (5, &base_row, base_color),
        (6, &base_row, base_color),
        (7, &base_row, base_color),
    ];

    for (offset, chars, color) in rows {
        let y = platform_top + offset;
        let line = Paragraph::new(chars.clone()).style(Style::default().fg(color));
        frame.render_widget(
            line,
            Rect {
                x: inner.x,
                y,
                width: inner.width,
                height: 1,
            },
        );
    }
}
