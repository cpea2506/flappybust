use crate::{
    constants::{GAME_SPEED, SCREEN_WIDTH},
    GameState,
};
use bevy::prelude::*;
use flappybust::{ternary, Math};
use itertools::Itertools;
use iyes_loopless::prelude::*;
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

use super::resources::DateTime;

#[derive(Component, Default)]
pub struct Pipe {
    pub size: Vec2,
    pub flip_y: bool,
    pub hidden: bool,

    translation: Vec3,
}

impl Pipe {
    pub const WIDTH: f32 = 52.;
    pub const HEIGHT: f32 = 320.;
    const GAP: f32 = 80.;

    fn new(x: f32, y: f32, flip_y: bool) -> Self {
        Pipe {
            translation: Vec3::new(x, y, 0.1),
            size: Vec2::new(Self::WIDTH, Self::HEIGHT),
            flip_y,
            ..default()
        }
    }

    fn generate_bundle(self, texture: &Handle<Image>) -> (SpriteBundle, Self) {
        (
            SpriteBundle {
                sprite: Sprite {
                    flip_y: self.flip_y,
                    ..default()
                },
                texture: texture.clone(),
                transform: Transform::from_translation(self.translation),
                ..default()
            },
            self,
        )
    }

    /// generate number of pipes by `num_pipe`
    #[inline]
    fn genrate_self(
        num_pipe: u32,
        first_time: bool,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        datetime: &Res<DateTime>,
    ) {
        let texture = asset_server.load(format!(
            "images/pipe_{}.png",
            match **datetime {
                DateTime::Day => "green",
                _ => "red",
            }
        ));

        let mut rng = thread_rng();
        let y_between = Uniform::new(-240., -50.);

        // spawn first 2 pipes
        (0..num_pipe).for_each(|i| {
            let pipe = Pipe::new(
                ternary!(first_time, SCREEN_WIDTH, SCREEN_WIDTH.half())
                    + Self::WIDTH.half()
                    + 175. * i as f32,
                y_between.sample(&mut rng),
                false,
            );
            let flipped_pipe = Pipe::new(
                pipe.translation.x,
                pipe.translation.y + Self::GAP + Self::HEIGHT,
                true,
            );

            commands.spawn_batch(vec![
                pipe.generate_bundle(&texture),
                flipped_pipe.generate_bundle(&texture),
            ]);
        });
    }
}

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Playing, spawn)
            .add_system(moving.run_in_state(GameState::Playing));
    }
}
fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, datetime: Res<DateTime>) {
    Pipe::genrate_self(2, true, &mut commands, &asset_server, &datetime);
}

fn moving(
    mut commands: Commands,
    mut pipe: Query<(Entity, &mut Transform), With<Pipe>>,
    asset_server: Res<AssetServer>,
    datetime: Res<DateTime>,
) {
    let half_pipe_width = Pipe::WIDTH.half();
    let half_screen_width = SCREEN_WIDTH.half();

    for ((pipe_entity, mut pipe_transform), (flipped_pipe_entity, mut flipped_pipe_transform)) in
        pipe.iter_mut().tuples()
    {
        pipe_transform.translation.x -= GAME_SPEED;
        flipped_pipe_transform.translation.x -= GAME_SPEED;

        // remove pipes that are outside of screen
        if pipe_transform.translation.x <= -half_pipe_width - half_screen_width {
            Pipe::genrate_self(1, false, &mut commands, &asset_server, &datetime);

            commands.entity(pipe_entity).despawn();
            commands.entity(flipped_pipe_entity).despawn();
        }
    }
}
