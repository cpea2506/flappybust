use crate::DateTime;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Background {
    pub x: f32,
    pub secondary: bool,
}

impl Background {
    pub fn spawn(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        nychthemeron: Res<DateTime>,
    ) {
        let texture: Handle<Image> = asset_server.load(&format!(
            "images/background-{:?}.png",
            nychthemeron.into_inner()
        ));

        commands
            .spawn_bundle(SpriteBundle {
                texture: texture.clone(),
                ..default()
            })
            .insert(Background::default());
        commands
            .spawn_bundle(SpriteBundle {
                texture,
                transform: Transform::from_xyz(288., 0., 0.),
                ..default()
            })
            .insert(Background {
                secondary: true,
                ..default()
            });
    }

    pub fn moving(mut background: Query<(&mut Background, &mut Transform)>) {
        for (mut background, mut transform) in &mut background {
            background.x = (background.x - 1.) % 288.;

            transform.translation.x = background.x + if background.secondary { 288. } else { 0. };
        }
    }
}
