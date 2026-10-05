use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::game::state::Game;

pub fn draw(frame: &mut Frame, game: &Game) {
    let size = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(size);

    draw_score(frame, chunks[0], game);
    draw_game_field(frame, chunks[1], game);
}

/// What the playing screen wants to happen next.
pub enum PlayingAction {
    Continue,
    Quit,
    // later: GameOver
}

pub fn handle_key(game: &mut Game, key: KeyCode) -> PlayingAction {
    match key {
        KeyCode::Char('q') => PlayingAction::Quit,
        _ => {
            let _ = game; // placeholder so compiler no complain
            PlayingAction::Continue
        }
    }
}

fn draw_score(frame: &mut Frame, area: Rect, game: &Game) {
    let text = Line::from(vec![
        Span::styled(" SCORE: ", Style::default().fg(Color::Yellow)),
        Span::styled(format!("{}", game.score), Style::default().fg(Color::White)),
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

fn draw_game_field(frame: &mut Frame, area: Rect, game: &Game) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Ground line: bottom row of inner
    let ground_y = inner.y + inner.height.saturating_sub(1);

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

    // Player position on screen
    let player_screen_y = ground_y.saturating_sub(1 + game.player_y);
    let player_screen_x = inner.x + game.player_x;

    if player_screen_x < inner.x + inner.width && player_screen_y >= inner.y {
        let player = Paragraph::new("@").style(
            Style::default()
                .fg(Color::LightRed)
                .add_modifier(Modifier::BOLD),
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
