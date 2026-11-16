use crate::{GAME_SPEED, SCREEN_WIDTH};
use bevy::prelude::*;

#[derive(Component)]
pub struct Background;

impl Background {
    /// Slower than the base and pipes so the distant city scrolls with parallax.
    pub const SPEED: f32 = GAME_SPEED / 3.0;
    /// Distance between the two background tiles; a pixel less than the width so they
    /// overlap, or rounding while scrolling opens a gap that shows as a black line.
    pub(super) const PERIOD: f32 = SCREEN_WIDTH - 1.0;
}
