use crate::{background::Background, DateTime};
use bevy::prelude::*;
use flappybust::Math;
use itertools::Itertools;
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

#[derive(Component)]
pub struct Pipe {
    pub translation: Vec3,
    pub has_passed: bool,
}

fn generate_pipes(commands: &mut Commands, pipe: Pipe, texture: &Handle<Image>) {
    let gap = 100.;
    let flipped_pipe = Pipe::new(pipe.translation.x, pipe.translation.y + gap + Pipe::HEIGHT);

    commands.spawn_batch(vec![
        (
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_translation(pipe.translation),
                ..default()
            },
            pipe,
        ),
        (
            SpriteBundle {
                sprite: Sprite {
                    flip_y: true,
                    ..default()
                },
                texture: texture.clone(),
                transform: Transform::from_translation(flipped_pipe.translation),
                ..default()
            },
            flipped_pipe,
        ),
    ]);
}

impl Pipe {
    pub const WIDTH: f32 = 52.;
    pub const HEIGHT: f32 = 320.;

    fn new(x: f32, y: f32) -> Self {
        Pipe {
            translation: Vec3::new(x, y, 0.1),
            has_passed: false,
        }
    }

    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, datetime: Res<DateTime>) {
        let mut rng = thread_rng();
        let y_between = Uniform::new(-240., -50.);
        let texture = asset_server.load(&format!(
            "images/pipe_{}.png",
            match datetime.into_inner() {
                DateTime::Day => "green",
                _ => "red",
            }
        ));

        // TODO: spawn first 3 pipe and generate more later
        // spawn first 1000 pipes
        (0..1000).for_each(|i| {
            let pipe = Pipe::new(360. + 175. * i as f32, y_between.sample(&mut rng));

            generate_pipes(&mut commands, pipe, &texture);
        });
    }

    pub fn moving(mut commands: Commands, mut pipe: Query<(Entity, &mut Transform), With<Pipe>>) {
        let half_pipe_width = Pipe::WIDTH.half();
        let half_background_width = Background::WIDTH.half();

        for (
            (pipe_entity, mut pipe_transform),
            (flipped_pipe_entity, mut flipped_pipe_transform),
        ) in pipe.iter_mut().tuples()
        {
            pipe_transform.translation.x -= 1.;
            flipped_pipe_transform.translation.x -= 1.;

            let outside_screen = -half_pipe_width - half_background_width;

            // remove pipes that are outside of screen
            if pipe_transform.translation.x <= outside_screen {
                commands.entity(pipe_entity).despawn();
                commands.entity(flipped_pipe_entity).despawn();
            }
        }
    }
}
