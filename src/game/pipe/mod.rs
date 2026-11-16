automod::dir!(pub "src/game/pipe");

use super::{DateTime, ImageAssets};
use crate::{GAME_SPEED, GameState, SCREEN_WIDTH};
use bevy::prelude::*;
use components::{Coin, Pipe};
use flappybust::frames;

// A respawned pair must start off screen.
const _: () = assert!(Pipe::PAIRS as f32 * Pipe::SPACING >= SCREEN_WIDTH + Pipe::WIDTH);

/// Pipe logic.
pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn)
            .add_systems(Update, moving.run_if(in_state(GameState::Playing)));
    }
}

fn spawn(mut commands: Commands, image_assets: Res<ImageAssets>, datetime: Res<DateTime>) {
    let texture = image_assets.pipe(*datetime);

    for i in 0..Pipe::PAIRS {
        let x = SCREEN_WIDTH + Pipe::WIDTH / 2.0 + Pipe::SPACING * i as f32;
        Pipe::spawn_pair(&mut commands, texture, &image_assets.coin, x);
    }
}

/// Entities that scroll with the pipes.
type Scrolling = Or<(With<Pipe>, With<Coin>)>;

fn moving(
    mut commands: Commands,
    mut pipes: Query<(Entity, Option<&Pipe>, &mut Transform), Scrolling>,
    image_assets: Res<ImageAssets>,
    datetime: Res<DateTime>,
    time: Res<Time>,
) {
    let distance = GAME_SPEED * frames(&time);

    for (entity, pipe, mut transform) in &mut pipes {
        transform.translation.x -= distance;

        // Remove pipes and coins that are outside of screen and queue a new pair behind the last one.
        if transform.translation.x <= -(SCREEN_WIDTH + Pipe::WIDTH) / 2.0 {
            commands.entity(entity).despawn();

            if pipe.is_some_and(|pipe| !pipe.flip_y) {
                let x = transform.translation.x + Pipe::PAIRS as f32 * Pipe::SPACING;
                let texture = image_assets.pipe(*datetime);
                Pipe::spawn_pair(&mut commands, texture, &image_assets.coin, x);
            }
        }
    }
}
