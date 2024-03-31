use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy)]
pub struct Background {
    pub translation: Vec3,
    pub secondary: bool,
}

impl Background {
    pub fn new(x: f32, y: f32, secondary: bool) -> Self {
        Background {
            translation: Vec3 { x, y, z: 0f32 },
            secondary,
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
