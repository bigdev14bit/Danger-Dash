/// Tunable physics constants. Change these to change the feel.
pub const GRAVITY: i32 = -1;         // applied every frame
pub const JUMP_VELOCITY: i32 = 4;    // initial upward velocity on jump
pub const MAX_JUMPS: u8 = 2;         // 1 = single jump, 2 = double jump
pub const TERMINAL_VELOCITY: i32 = -5; // fastest fall speed (negative = down)

/// The player character.
///
/// `y` na vertical position with 0 = standing on the ground.
/// Positive y = floating up. `velocity_y` positive = rising,
/// negative = falling.
pub struct Player {
    pub x: u16,
    pub y: i32,           // signed now — can go negative when falling off
    pub velocity_y: i32,
    pub on_ground: bool,
    pub jumps_left: u8,
}

impl Player {
    pub fn new(x: u16) -> Self {
        Self {
            x,
            y: 0,
            velocity_y: 0,
            on_ground: true,
            jumps_left: MAX_JUMPS,
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self, max_x: u16) {
        if self.x < max_x {
            self.x += 1;
        }
    }

    /// Try to jump. E go succeed if:
    ///   - we get jumps left (either on ground or mid-air for double jump)
    /// Returns true if the jump fired, false otherwise.
    pub fn try_jump(&mut self) -> bool {
        if self.jumps_left == 0 {
            return false;
        }
        self.velocity_y = JUMP_VELOCITY;
        self.jumps_left -= 1;
        self.on_ground = false;
        true
    }

    /// Advance physics by one frame.
    /// `ground_y` na the world y-coordinate of the platform top (0 = ground level).
    pub fn tick(&mut self, ground_y: i32) {
        // Apply gravity
        self.velocity_y += GRAVITY;

        // Clamp falling speed (so e no accelerate forever)
        if self.velocity_y < TERMINAL_VELOCITY {
            self.velocity_y = TERMINAL_VELOCITY;
        }

        // Integrate position
        self.y += self.velocity_y;

        // Land on ground
        if self.y <= ground_y {
            self.y = ground_y;
            self.velocity_y = 0;
            if !self.on_ground {
                // Just landed — restore jumps
                self.on_ground = true;
                self.jumps_left = MAX_JUMPS;
            }
        } else {
            // In the air
            self.on_ground = false;
        }
    }
}
