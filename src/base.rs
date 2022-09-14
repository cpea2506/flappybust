use bevy::prelude::*;
use flappybust::Math;

use crate::background::Background;

#[derive(Component, Default)]
pub struct Base {
    pub translation: Vec3,
    pub secondary: bool,
}

impl Base {
    fn new(x: f32, y: f32, secondary: bool) -> Self {
        Base {
            translation: Vec3::new(x, y, 0.2),
            secondary,
        }
    }

    pub fn height() -> f32 {
        112.
    }

    pub fn width() -> f32 {
        336.
    }

    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        let texture = asset_server.load("images/base.png");

        let base = Base::new(
            0.,
            Base::height().half() - Background::height().half(),
            false,
        );
        let secondary_base = Base::new(Base::width(), base.translation.y, true);

        commands
            .spawn_bundle(SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_translation(base.translation),
                ..default()
            })
            .insert(base);

        commands
            .spawn_bundle(SpriteBundle {
                texture,
                transform: Transform::from_translation(secondary_base.translation),
                ..default()
            })
            .insert(secondary_base);
    }

    pub fn moving(mut base: Query<(&mut Base, &mut Transform), With<Base>>) {
        let base_width = Base::width();

        for (mut base, mut transform) in &mut base {
            base.translation.x = (base.translation.x - 1.) % (base_width - 24.);

            transform.translation.x =
                base.translation.x + if base.secondary { base_width - 24. } else { 0. };
        }
    }
}
