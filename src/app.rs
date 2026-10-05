use crossterm::event::KeyCode;

use crate::game::state::Game;
use crate::screens::menu::{self, Menu, MenuAction};
use crate::screens::playing::{self, PlayingAction};

/// Which screen are we currently showing?
pub enum Screen {
    Menu(Menu),
    Playing(Game),
    // later: GameOver, HowToPlay
}

pub struct App {
    pub screen: Screen,
    /// Set to true when we want to exit the program.
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Menu(Menu::new()),
            should_quit: false,
        }
    }

    /// Handle a key press at the app level, dispatching to the current screen.
    pub fn handle_key(&mut self, key: KeyCode) {
        // We need to borrow screen mutably but also reassign it.
        // Trick: take ownership temporarily with a placeholder.
        let current = std::mem::replace(&mut self.screen, Screen::Menu(Menu::new()));

        self.screen = match current {
            Screen::Menu(mut menu) => match menu::handle_key(&mut menu, key) {
                MenuAction::Play => Screen::Playing(Game::new()),
                MenuAction::HowToPlay => {
                    // placeholder: for now just go back to menu
                    Screen::Menu(menu)
                }
                MenuAction::Quit => {
                    self.should_quit = true;
                    Screen::Menu(menu)
                }
                MenuAction::None => Screen::Menu(menu),
            },
            Screen::Playing(mut game) => match playing::handle_key(&mut game, key) {
                PlayingAction::Quit => {
                    // back to menu
                    Screen::Menu(Menu::new())
                }
                PlayingAction::Continue => Screen::Playing(game),
            },
        };
    }
}
