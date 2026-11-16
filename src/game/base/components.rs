use super::super::layer;
use crate::SCREEN_HEIGHT;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Base {
    pub(super) translation: Vec3,
    pub(super) secondary: bool,
}

impl Base {
    pub const WIDTH: f32 = 336f32;
    pub const HEIGHT: f32 = 112f32;
    pub(super) const RESET_POINT: f32 = Self::WIDTH - 24f32;
    /// Top edge, where the bird lands.
    pub const TOP: f32 = Self::HEIGHT - SCREEN_HEIGHT / 2.0;

    pub fn new(x: f32, y: f32, secondary: bool) -> Self {
        Base {
            translation: Vec3::new(x, y, layer::BASE),
            secondary,
        }
    }

    #[inline]
    pub fn generate_bundle(self, texture: &Handle<Image>) -> (Sprite, Transform, Self) {
        (
            Sprite::from_image(texture.clone()),
            Transform::from_translation(self.translation),
            self,
        )
    }
}
