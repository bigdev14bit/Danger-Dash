use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// The menu items.
const ITEMS: &[&str] = &["PLAY", "HOW TO PLAY", "QUIT"];

/// Menu state: which item is currently selected.
pub struct Menu {
    pub selected: usize,
}

impl Menu {
    pub fn new() -> Self {
        Self { selected: 0 }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = ITEMS.len() - 1; // wrap to bottom
        }
    }

    pub fn move_down(&mut self) {
        if self.selected < ITEMS.len() - 1 {
            self.selected += 1;
        } else {
            self.selected = 0; // wrap to top
        }
    }
}

pub enum MenuAction {
    Play,
    HowToPlay,
    Quit,
    None,
}

pub fn handle_key(menu: &mut Menu, key: KeyCode) -> MenuAction {
    match key {
        KeyCode::Up | KeyCode::Char('k') => {
            menu.move_up();
            MenuAction::None
        }
        KeyCode::Down | KeyCode::Char('j') => {
            menu.move_down();
            MenuAction::None
        }
        KeyCode::Enter => match menu.selected {
            0 => MenuAction::Play,
            1 => MenuAction::HowToPlay,
            2 => MenuAction::Quit,
            _ => MenuAction::None,
        },
        KeyCode::Char('q') => MenuAction::Quit,
        _ => MenuAction::None,
    }
}

pub fn draw(frame: &mut Frame, menu: &Menu) {
    let size = frame.area();

    // Full-screen bordered block
    let outer = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(" DANGER DASH ");
    let inner = outer.inner(size);
    frame.render_widget(outer, size);

    // Vertical layout inside: title area, spacer, menu items
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // title
            Constraint::Length(2), // spacer
            Constraint::Length(ITEMS.len() as u16 * 2), // items
            Constraint::Min(0),    // rest
            Constraint::Length(2), // help footer
        ])
        .split(inner);

    draw_title(frame, chunks[0]);
    draw_items(frame, chunks[2], menu);
    draw_footer(frame, chunks[4]);
}

fn draw_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "DANGER",
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(
                "DASH",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ])
    .alignment(Alignment::Center);
    frame.render_widget(title, area);
}

fn draw_items(frame: &mut Frame, area: Rect, menu: &Menu) {
    let mut lines = Vec::new();
    for (i, item) in ITEMS.iter().enumerate() {
        let is_selected = i == menu.selected;
        let marker = if is_selected { "▶ " } else { "  " };
        let style = if is_selected {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        lines.push(Line::from(vec![
            Span::styled(marker, style),
            Span::styled(*item, style),
        ]));
        lines.push(Line::from("")); // spacing between items
    }
    let widget = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(widget, area);
}

fn draw_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new(" ↑/↓ to move   Enter to select   q to quit ")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(footer, area);
}
