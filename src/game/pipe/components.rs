use super::super::layer;
use crate::GameState;
use bevy::prelude::*;
use std::ops::Range;
use rand::{random_bool, random_range};

#[derive(Component)]
pub struct Pipe;

/// The lower pipe of a pair, which scoring and respawning go by.
#[derive(Component)]
pub struct LowerPipe;

/// Marks a lower pipe the bird has already scored on.
#[derive(Component)]
pub struct Scored;

impl Pipe {
    pub const WIDTH: f32 = 52f32;
    pub const HEIGHT: f32 = 320f32;
    // Spacing and gap height range.
    const GAP: f32 = 85f32;
    /// Horizontal distance between two pipe pairs.
    pub(super) const SPACING: f32 = 144f32;
    /// Pairs alive at once: enough to span the screen plus a pipe, so respawns start off screen.
    pub(super) const PAIRS: usize = 3;
    /// Range of the lower pipe's center y, which places the gap.
    const LOWER_Y: Range<f32> = -225f32..-84f32;

    /// Spawn a lower and an upper pipe at `x` with a random gap height,
    /// maybe with a coin in the gap.
    pub(super) fn spawn_pair(
        commands: &mut Commands,
        texture: &Handle<Image>,
        coin: &Handle<Image>,
        x: f32,
    ) {
        let y = random_range(Self::LOWER_Y);

        let pipe = |y, flip_y| {
            (
                Sprite {
                    image: texture.clone(),
                    flip_y,
                    ..default()
                },
                Transform::from_xyz(x, y, layer::PIPE),
                Pipe,
                DespawnOnExit(GameState::Over),
            )
        };
        commands.spawn((pipe(y, false), LowerPipe));
        commands.spawn(pipe(y + Self::GAP + Self::HEIGHT, true));

        if random_bool(Coin::CHANCE) {
            // Anywhere in the gap, fully clear of both pipes.
            let gap_bottom = y + Self::HEIGHT / 2.0;
            let y = gap_bottom + random_range(Coin::SIZE / 2.0..Self::GAP - Coin::SIZE / 2.0);

            commands.spawn((
                Sprite {
                    image: coin.clone(),
                    // First frame of the vertical sprite sheet.
                    rect: Some(Rect::new(0.0, 0.0, Coin::SIZE, Coin::SIZE)),
                    ..default()
                },
                Transform::from_xyz(x, y, layer::COIN),
                Coin,
                DespawnOnExit(GameState::Over),
            ));
        }
    }
}

#[derive(Component)]
pub struct Coin;

impl Coin {
    pub const SIZE: f32 = 16f32;
    /// Frames stacked vertically in the sprite sheet.
    pub(super) const FRAMES: u32 = 10;
    pub(super) const FRAME_SECS: f32 = 0.08;
    /// Chance that a pipe pair has a coin.
    const CHANCE: f64 = 0.5;
}
