use crate::{background::Background, DateTime};
use bevy::prelude::*;
use flappybust::Math;
use itertools::Itertools;
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

#[derive(Component, Debug)]
pub struct Pipe {
    pub translation: Vec3,
}

impl Pipe {
    fn new(x: f32, y: f32) -> Self {
        Pipe {
            translation: Vec3::new(x, y, 0.1),
        }
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

    fn generate_pipes(commands: &mut Commands, texture: &Handle<Image>, pipe: Pipe) {
        let gap = 80.;
        let flipped_pipe = Pipe::new(
            pipe.translation.x,
            pipe.translation.y + gap + Pipe::height(),
        );

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
        let y_between = Uniform::new(-240., -50.);
        let texture = Pipe::texture(&asset_server, &datetime);

        // spawn first three pipes
        (0..1000).for_each(|i| {
            let pipe = Pipe::new(194. + 175. * i as f32, y_between.sample(&mut rng));

            Pipe::generate_pipes(&mut commands, &texture, pipe);
        });
    }

    pub fn moving(mut commands: Commands, mut pipe: Query<(Entity, &mut Transform), With<Pipe>>) {
        let half_pipe_width = Pipe::width().half();
        let half_background_width = Background::width().half();

        for (
            (pipe_entity, mut pipe_transform),
            (flipped_pipe_entity, mut flipped_pipe_transform),
        ) in pipe.iter_mut().tuples()
        {
            pipe_transform.translation.x -= 1.;
            flipped_pipe_transform.translation.x -= 1.;

            let outside_screen = -half_pipe_width - half_background_width;

            // pipes are outside of screen
            if pipe_transform.translation.x <= outside_screen {
                // remove pipes
                commands.entity(pipe_entity).despawn();
                commands.entity(flipped_pipe_entity).despawn();
            }
        }
    }
}
