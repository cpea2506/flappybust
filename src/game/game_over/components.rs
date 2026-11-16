use bevy::prelude::*;

#[derive(Component)]
pub struct Medal;

impl Medal {
    pub const POSITION: Vec2 = Vec2::new(-65f32, 47f32);
    /// Scale the medal starts at before shrinking into place.
    pub const START_SCALE: f32 = 25f32;
    /// Scale lost per frame.
    pub const SHRINK_SPEED: f32 = 2f32;
}

#[derive(Component)]
pub struct RestartButton;

impl RestartButton {
    pub const SIZE: Vec2 = Vec2::new(114f32, 40f32);
    pub const Y: f32 = -35f32;
}

#[derive(Component)]
pub struct Scoreboard {
    pub velocity: f32,
    pub gravity: f32,
}

impl Scoreboard {
    /// Off screen, below the base.
    pub const START_Y: f32 = -199f32;
    pub const END_Y: f32 = 57f32;
}

impl Default for Scoreboard {
    fn default() -> Self {
        Scoreboard {
            gravity: 0.15,
            velocity: 0f32,
        }
    }
}

#[derive(Component)]
pub struct GameOverText {
    pub velocity: f32,
    pub gravity: f32,

    /// Whether text should be bounced or not (default: true).
    pub bounce: bool,
}

impl GameOverText {
    /// Off screen, above the top.
    pub const START_Y: f32 = 351f32;
    pub const REST_Y: f32 = 156f32;
    /// Share of speed kept after bouncing off the rest position.
    pub const BOUNCINESS: f32 = 0.73;
}

impl Default for GameOverText {
    fn default() -> Self {
        GameOverText {
            gravity: 0.6,
            velocity: 0f32,
            bounce: true,
        }
    }
}
