automod::dir!(pub "src/game/score");

use super::layer;
use super::{
    AudioAssets, FontAssets, GameState, audio::messages::AudioMessage, bird::components::Bird,
    game_over::messages::ScoreboardDisplayed, pipe::components::Pipe,
};
use crate::SCREEN_HEIGHT;
use bevy::{camera::visibility::NoFrustumCulling, prelude::*, sprite::Anchor};
use components::*;
use flappybust::despawn;

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
            .add_systems(
                Update,
                (
                    (record, display_current_score).run_if(in_state(GameState::Playing)),
                    display_scoreboard_score.run_if(in_state(GameState::Over)),
                ),
            )
            .add_systems(OnEnter(GameState::Over), despawn::<CurrentScore>)
            .add_systems(OnExit(GameState::Over), despawn::<ScoreboardScore>);
    }
}

fn spawn_current_score(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    prev_score: Res<Score>,
) {
    let score = Score {
        current: 0,
        ..*prev_score
    };

    commands.spawn((
        Text2d::new(score.current.to_string()),
        TextFont {
            font: font_assets.teko_bold.clone().into(),
            font_size: FontSize::Px(CURRENT_SCORE_FONT_SIZE),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        Anchor::TOP_CENTER,
        Transform::from_xyz(0f32, SCREEN_HEIGHT / 2.0, layer::HUD),
        // Frustum culling wrongly hides text whose top sits on the screen edge,
        // and this text is always on screen anyway.
        NoFrustumCulling,
        CurrentScore,
    ));

    commands.insert_resource(score);
}

fn display_current_score(
    score: Res<Score>,
    mut current_score: Single<&mut Text2d, With<CurrentScore>>,
) {
    current_score.0 = score.current.to_string();
}

fn display_scoreboard_score(
    mut commands: Commands,
    score: Res<Score>,
    font_assets: Res<FontAssets>,
    scoreboard_displayed: MessageReader<ScoreboardDisplayed>,
    spawned: Query<(), With<ScoreboardScore>>,
) {
    // The scoreboard reports itself displayed every frame, so only spawn once.
    if scoreboard_displayed.is_empty() || !spawned.is_empty() {
        return;
    }

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
            ScoreboardScore,
        ));
    }
}

fn record(
    audio_assets: Res<AudioAssets>,
    mut audio_message: MessageWriter<AudioMessage>,
    bird_transform: Single<&Transform, With<Bird>>,
    mut pipes: Query<(&mut Pipe, &Transform)>,
    mut score: ResMut<Score>,
) {
    for (mut pipe, pipe_transform) in &mut pipes {
        // Increase score each time the bird has passed a pipe pair (counted on the lower pipe).
        if !pipe.flip_y
            && !pipe.scored
            && bird_transform.translation.x + Bird::WIDTH / 2.0 > pipe_transform.translation.x
        {
            score.current += 1;
            score.highest = score.current.max(score.highest);
            pipe.scored = true;

            audio_message.write(AudioMessage::new(&audio_assets.score, false));
        }
    }
}
