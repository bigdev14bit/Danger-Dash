use super::player::Player;

/// Everything we need to know about the world right now.
pub struct Game {
    pub player: Player,
    pub score: u32,
    pub field_width: u16,
    pub field_height: u16,   // NEW: we need this for physics bounds
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(10),
            score: 0,
            field_width: 80,
            field_height: 24,
        }
    }
}