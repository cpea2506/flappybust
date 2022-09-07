use std::{thread::sleep, time::Duration};

use crate::{base::Base, DateTime};
use bevy::{prelude::*, transform};
use flappybust::Math;
use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
    thread_rng,
};

use crate::GameState;

#[derive(Component)]
pub struct Pipe {
    pub translation: Vec3,
    entity: Option<Entity>,
}

impl Pipe {
    fn new(x: f32, y: f32) -> Self {
        Pipe {
            translation: Vec3::new(x, y, 0.1),
            entity: None,
        }
    }

    fn insert_entity(self, entity: Option<Entity>) -> Self {
        Pipe { entity, ..self }
    }

    pub fn height() -> f32 {
        320.
    }

    pub fn width() -> f32 {
        52.
    }

    fn texture(asset_server: &AssetServer, datetime: &DateTime) -> Handle<Image> {
        asset_server.load(&format!(
            "images/pipe-{color}.png",
            color = match datetime {
                DateTime::Day => "green",
                DateTime::Night => "red",
            }
        ))
    }

    fn y_between() -> Uniform<f32> {
        Uniform::new(-240.0, -50.)
    }

    fn generate_pipes(commands: &mut Commands, texture: &Handle<Image>, pipe: Pipe) {
        let mut gap = 400.;
        let flipped_pipe = Pipe::new(pipe.translation.x, pipe.translation.y + gap);

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

    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, datetime: Res<DateTime>) {
        let mut rng = thread_rng();
        let y_between = Pipe::y_between();
        let texture = Pipe::texture(&asset_server, &datetime);

        // spawn first three pipes
        (0..3).for_each(|i| {
            let pipe = Pipe::new(194. + 175.0 * i as f32, y_between.sample(&mut rng));

            Pipe::generate_pipes(&mut commands, &texture, pipe);
        });
    }

    pub fn moving(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        datetime: Res<DateTime>,
        mut pipe: Query<(Entity, &mut Transform), With<Pipe>>,
    ) {
        let mut rng = thread_rng();
        let y_between = Pipe::y_between();
        let texture = Pipe::texture(&asset_server, &datetime);

        let mut iter = pipe.iter_mut().peekable();

        while let Some((entity, mut transform)) = iter.next() {
            transform.translation.x -= 1.;

            // pipes are outside of screen
            if transform.translation.x <= -Pipe::width().half() - Base::width().half() {
                // create new pair of pipes with
                // value of last pipe in the current screen
                if iter.peek().is_none() {
                    (1..3).for_each(|i| {
                        let pipe = Pipe::new(
                            194. + 175. * i as f32 + transform.translation.x,
                            y_between.sample(&mut rng),
                        );

                        Pipe::generate_pipes(&mut commands, &texture, pipe);
                    })
                }

                commands.entity(entity).despawn();
            }
        }
    }
}
