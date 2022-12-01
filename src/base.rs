use crate::background::Background;
use bevy::prelude::*;
use flappybust::{ternary, Math};

#[derive(Component, Default)]
pub struct Base {
    pub translation: Vec3,
    pub secondary: bool,
}

impl Base {
    pub const WIDTH: f32 = 336.;
    pub const HEIGHT: f32 = 112.;
    const LIMIT: f32 = Base::WIDTH - 24.;

    fn new(x: f32, y: f32, secondary: bool) -> Self {
        Base {
            translation: Vec3::new(x, y, 0.2),
            secondary,
        }
    }

    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        let base = Base::new(0., Base::HEIGHT.half() - Background::HEIGHT.half(), false);
        let secondary_base = Base::new(Base::WIDTH, base.translation.y, true);
        let texture = asset_server.load("images/base.png");

        commands.spawn_batch(vec![
            (
                SpriteBundle {
                    texture: texture.clone(),
                    transform: Transform::from_translation(base.translation),
                    ..default()
                },
                base,
            ),
            (
                SpriteBundle {
                    texture,
                    transform: Transform::from_translation(secondary_base.translation),
                    ..default()
                },
                secondary_base,
            ),
        ]);
    }

    pub fn moving(mut base: Query<(&mut Base, &mut Transform)>) {
        for (mut base, mut transform) in &mut base {
            base.translation.x = (base.translation.x - 1.) % Base::LIMIT;
            transform.translation.x =
                base.translation.x + ternary!(base.secondary, Base::LIMIT, 0.);
        }
    }
}
