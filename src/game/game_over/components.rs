use bevy::prelude::*;

#[derive(Component)]
pub struct Medal;

#[derive(Component)]
pub struct RestartButton;

impl RestartButton {
    pub const SIZE: Vec2 = Vec2::new(114f32, 40f32);
}

#[derive(Component)]
pub struct Scoreboard {
    pub velocity: f32,
    pub gravity: f32,
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

impl Default for GameOverText {
    fn default() -> Self {
        GameOverText {
            gravity: 0.1,
            velocity: 0f32,
            bounce: true,
        }
    }
}
