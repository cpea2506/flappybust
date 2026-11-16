use crate::SCREEN_HEIGHT;

/// The scrolling ground.
pub struct Base;

impl Base {
    pub const WIDTH: f32 = 336f32;
    pub const HEIGHT: f32 = 112f32;
    /// Distance between the two base tiles; less than the width so they overlap.
    pub(super) const PERIOD: f32 = Self::WIDTH - 24f32;
    /// Top edge, where the bird lands.
    pub const TOP: f32 = Self::HEIGHT - SCREEN_HEIGHT / 2.0;
}
