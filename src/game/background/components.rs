use crate::GAME_SPEED;
use super::super::layer;
use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy)]
pub struct Background {
    pub translation: Vec3,
    pub secondary: bool,
}

impl Background {
    /// Slower than the base and pipes so the distant city scrolls with parallax.
    pub const SPEED: f32 = GAME_SPEED / 3.0;

    pub fn new(x: f32, y: f32, secondary: bool) -> Self {
        Background {
            translation: Vec3::new(x, y, layer::BACKGROUND),
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
