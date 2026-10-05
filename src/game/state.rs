/// Everything we need to know about the world right now.
pub struct Game {
    pub player_x: u16,
    pub player_y: u16,
    pub score: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player_x: 10,
            player_y: 0,
            score: 0,
        }
    }
}
