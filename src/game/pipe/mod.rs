automod::dir!(pub "src/game/pipe");

use super::{DateTime, ImageAssets};
use crate::{GameState, SCREEN_WIDTH};
use bevy::prelude::*;
use components::Pipe;
use flappybust::{despawn, BasicMath};
use itertools::Itertools;

/// Pipe logic.
pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Ready), despawn::<Pipe>)
            .add_systems(OnEnter(GameState::Playing), spawn)
            .add_systems(Update, moving.run_if(in_state(GameState::Playing)));
    }
}
fn spawn(mut commands: Commands, image_assets: Res<ImageAssets>, datetime: Res<DateTime>) {
    Pipe::genrate_self(2, true, &mut commands, &image_assets, &datetime);
}

fn moving(
    mut commands: Commands,
    mut pipe: Query<(Entity, &mut Transform), With<Pipe>>,
    image_assets: Res<ImageAssets>,
    datetime: Res<DateTime>,
) {
    let half_pipe_width = Pipe::WIDTH.half();
    let half_screen_width = SCREEN_WIDTH.half();

    for ((pipe_entity, mut pipe_transform), (flipped_pipe_entity, mut flipped_pipe_transform)) in
        pipe.iter_mut().tuples()
    {
        pipe_transform.translation.x -= 1f32;
        flipped_pipe_transform.translation.x -= 1f32;

        // Remove pipes that are outside of screen.
        if pipe_transform.translation.x <= -half_pipe_width - half_screen_width {
            Pipe::genrate_self(1, false, &mut commands, &image_assets, &datetime);

            commands.entity(pipe_entity).despawn();
            commands.entity(flipped_pipe_entity).despawn();
        }
    }
}
