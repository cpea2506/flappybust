use crate::DateTime;
use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy)]
pub struct Background {
    pub translation: Vec3,
    pub secondary: bool,
}

impl Background {
    fn new(x: f32, y: f32, secondary: bool) -> Self {
        Background {
            translation: Vec3 { x, y, z: 0. },
            secondary,
        }
    }

    pub fn height() -> f32 {
        512.
    }

    pub fn width() -> f32 {
        288.
    }

    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, datetime: Res<DateTime>) {
        let texture: Handle<Image> = asset_server.load(&format!(
            "images/bg_{datetime}.png",
            datetime = match datetime.into_inner() {
                DateTime::Day => "day",
                DateTime::Night => "night",
            }
        ));

        let background = Background::default();
        let secondary_background =
            Background::new(background.translation.x, background.translation.y, true);

        commands
            .spawn_bundle(SpriteBundle {
                texture: texture.clone(),
                ..default()
            })
            .insert(background);

        commands
            .spawn_bundle(SpriteBundle {
                texture,
                transform: Transform::from_translation(secondary_background.translation),
                ..default()
            })
            .insert(secondary_background);
    }

    pub fn moving(mut background: Query<(&mut Background, &mut Transform)>) {
        let background_width = Background::width();

        for (mut background, mut transform) in &mut background {
            background.translation.x = (background.translation.x - 1.) % background_width;

            transform.translation.x = background.translation.x
                + if background.secondary {
                    Background::width()
                } else {
                    0.
                };
        }
    }
}
