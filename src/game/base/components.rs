use bevy::prelude::*;
use flappybust::BasicMath;

#[derive(Component, Default)]
pub struct Base {
    pub collider_pos: f32,

    pub(super) translation: Vec3,
    pub(super) secondary: bool,
}

impl Base {
    pub const WIDTH: f32 = 336f32;
    pub const HEIGHT: f32 = 112f32;
    pub(super) const RESET_POINT: f32 = Self::WIDTH - 24f32;

    pub fn new(x: f32, y: f32, secondary: bool) -> Self {
        Base {
            translation: Vec3::new(x, y, 0.4),
            secondary,
            collider_pos: y + Self::HEIGHT.half(),
        }
    }

    #[inline]
    pub fn generate_bundle(self, texture: &Handle<Image>) -> (SpriteBundle, Self) {
        (
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_translation(self.translation),
                ..default()
            },
            self,
        )
    }
}
