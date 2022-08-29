use bevy::{math::XY, prelude::*};
use rand::{random, thread_rng, Rng};

use crate::GameState;

#[derive(Component, Default)]
pub struct Pipe {
    pub translation: Vec3,
}

impl Pipe {
    fn new(x: f32, y: f32) -> Self {
        Pipe {
            translation: Vec3::new(x, y, 0.1),
        }
    }

    fn height() -> f32 {
        320.
    }

    fn width() -> f32 {
        52.
    }

    fn gap() -> f32 {
        400.
    }

    pub fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
        let mut rng = thread_rng();
        let texture: Handle<Image> = asset_server.load("images/pipe-green.png");

        for i in 0..3 {
            let pipe = Pipe::new(170. + 175.0 * i as f32, rng.gen_range(-240.0..=-120.));
            let flipped_pipe = Pipe::new(pipe.translation.x, pipe.translation.y + Pipe::gap());

            commands
                .spawn_bundle(SpriteBundle {
                    texture: texture.clone(),
                    transform: Transform::from_translation(pipe.translation),
                    ..default()
                })
                .insert(pipe);

            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        flip_y: true,
                        ..default()
                    },
                    texture: texture.clone(),
                    transform: Transform::from_translation(flipped_pipe.translation),
                    ..default()
                })
                .insert(flipped_pipe);
        }
    }

    pub fn moving_system(mut query: Query<(&mut Pipe, &mut Transform)>) {
        for (mut pipe, mut transform) in query.iter_mut() {
            transform.translation.x -= 1.;
        }
    }
}
