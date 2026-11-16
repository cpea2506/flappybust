automod::dir!(pub "src/game/pipe");

use super::{DateTime, ImageAssets};
use crate::{GAME_SPEED, GameState, SCREEN_WIDTH};
use bevy::prelude::*;
use components::Pipe;
use flappybust::{despawn, frames};

/// Pipe logic.
pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn)
            .add_systems(Update, moving.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Over), despawn::<Pipe>);
    }
}

fn spawn(mut commands: Commands, image_assets: Res<ImageAssets>, datetime: Res<DateTime>) {
    let texture = image_assets.pipe(*datetime);

    for i in 0..2 {
        let x = SCREEN_WIDTH + Pipe::WIDTH / 2.0 + Pipe::SPACING * i as f32;
        Pipe::spawn_pair(&mut commands, texture, x);
    }
}

fn moving(
    mut commands: Commands,
    mut pipes: Query<(Entity, &Pipe, &mut Transform)>,
    image_assets: Res<ImageAssets>,
    datetime: Res<DateTime>,
    time: Res<Time>,
) {
    let distance = GAME_SPEED * frames(&time);

    for (entity, pipe, mut transform) in &mut pipes {
        transform.translation.x -= distance;

        // Remove pipes that are outside of screen and queue a new pair behind the last one.
        if transform.translation.x <= -(SCREEN_WIDTH + Pipe::WIDTH) / 2.0 {
            commands.entity(entity).despawn();

            if !pipe.flip_y {
                let x = transform.translation.x + 2.0 * Pipe::SPACING;
                Pipe::spawn_pair(&mut commands, image_assets.pipe(*datetime), x);
            }
        }
    }
}
