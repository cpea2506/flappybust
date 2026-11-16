automod::dir!(pub "src/game/score");

use super::{
    AudioAssets, FontAssets, GameState, bird::components::Bird, layer, pipe::components::Pipe,
};
use crate::{GameOverPhase, SCREEN_HEIGHT};
use bevy::{camera::visibility::NoFrustumCulling, prelude::*, sprite::Anchor};
use components::*;
use flappybust::sound;

/// Bevy 0.15 text renders 1.2x larger, so font sizes are scaled down to keep the old size.
const TEXT_SCALE: f32 = 1.2;
const CURRENT_SCORE_FONT_SIZE: f32 = 64f32 / TEXT_SCALE;
const SCOREBOARD_FONT_SIZE: f32 = 40f32 / TEXT_SCALE;
/// Right edge of the scoreboard scores, aligned with the "SCORE" and "BEST" labels.
const SCOREBOARD_SCORE_RIGHT: f32 = 100f32;
const SCOREBOARD_CURRENT_Y: f32 = 65.5;
const SCOREBOARD_BEST_Y: f32 = 16f32;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(OnEnter(GameState::Playing), spawn_current_score)
            .add_systems(Update, record.run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Over), save_highest)
            .add_systems(OnEnter(GameOverPhase::Medal), spawn_scoreboard_score);
    }
}

fn spawn_current_score(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    mut score: ResMut<Score>,
) {
    score.current = 0;

    commands.spawn((
        Text2d::new("0"),
        TextFont {
            font: font_assets.teko_bold.clone().into(),
            font_size: FontSize::Px(CURRENT_SCORE_FONT_SIZE),
            ..default()
        },
        Anchor::TOP_CENTER,
        Transform::from_xyz(0f32, SCREEN_HEIGHT / 2.0, layer::HUD),
        // Frustum culling wrongly hides text whose top sits on the screen edge,
        // and this text is always on screen anyway.
        NoFrustumCulling,
        CurrentScore,
        DespawnOnExit(GameState::Playing),
    ));
}

fn spawn_scoreboard_score(mut commands: Commands, score: Res<Score>, font_assets: Res<FontAssets>) {
    let text_font = TextFont {
        font: font_assets.teko_bold.clone().into(),
        font_size: FontSize::Px(SCOREBOARD_FONT_SIZE),
        ..default()
    };

    // Right-aligned to the "SCORE" and "BEST" labels.
    for (value, y) in [
        (score.current, SCOREBOARD_CURRENT_Y),
        (score.highest, SCOREBOARD_BEST_Y),
    ] {
        commands.spawn((
            Text2d::new(value.to_string()),
            text_font.clone(),
            Anchor::CENTER_RIGHT,
            Transform::from_xyz(SCOREBOARD_SCORE_RIGHT, y, layer::SCOREBOARD_SCORE),
            DespawnOnExit(GameState::Over),
        ));
    }
}

/// Score a point each time the bird passes a pipe pair (counted on the lower pipe).
fn record(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    bird: Single<&Transform, With<Bird>>,
    mut pipes: Query<(&mut Pipe, &Transform)>,
    mut score: ResMut<Score>,
    mut current_score: Single<&mut Text2d, With<CurrentScore>>,
) {
    for (mut pipe, pipe_transform) in &mut pipes {
        if !pipe.flip_y
            && !pipe.scored
            && bird.translation.x + Bird::WIDTH / 2.0 > pipe_transform.translation.x
        {
            pipe.scored = true;
            score.current += 1;
            score.highest = score.current.max(score.highest);
            current_score.0 = score.current.to_string();

            commands.spawn(sound(&audio_assets.score));
        }
    }
}

/// Key of the best score in browser local storage.
///
/// On desktop the best score only lasts for the game session.
#[cfg(target_arch = "wasm32")]
const HIGHEST_KEY: &str = "flappybust_best";

fn save_highest(score: Res<Score>) {
    if score.current == score.highest && score.current > 0 {
        store_highest(score.highest);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn load_highest() -> usize {
    0
}

#[cfg(not(target_arch = "wasm32"))]
fn store_highest(_highest: usize) {}

#[cfg(target_arch = "wasm32")]
fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

#[cfg(target_arch = "wasm32")]
fn load_highest() -> usize {
    local_storage()
        .and_then(|storage| storage.get_item(HIGHEST_KEY).ok()?)
        .and_then(|best| best.parse().ok())
        .unwrap_or(0)
}

#[cfg(target_arch = "wasm32")]
fn store_highest(highest: usize) {
    if let Some(storage) = local_storage()
        && storage.set_item(HIGHEST_KEY, &highest.to_string()).is_err()
    {
        warn!("Failed to save best score");
    }
}
