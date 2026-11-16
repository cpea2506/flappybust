use crate::GAME_SPEED;
use bevy::prelude::*;

#[derive(Component)]
pub struct Background;

impl Background {
    /// Slower than the base and pipes so the distant city scrolls with parallax.
    pub const SPEED: f32 = GAME_SPEED / 3.0;
}
