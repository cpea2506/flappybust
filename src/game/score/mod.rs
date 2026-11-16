automod::dir!(pub "src/game/score");

use super::{
    AudioAssets, FontAssets, GameState, audio::events::AudioEvent, bird::components::Bird,
    game_over::events::ScoreboardDisplayed, pipe::components::Pipe,
};
use crate::SCREEN_HEIGHT;
use bevy::{prelude::*, sprite::Anchor};
use components::*;
use flappybust::despawn;

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
        // Bevy 0.15 text renders 1.2x larger, so scale down to keep the old size.
        TextFont {
            font: font_assets.teko_bold.clone().into(),
            font_size: FontSize::Px(64f32 / 1.2),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        Anchor::TOP_CENTER,
        Transform::from_xyz(0f32, SCREEN_HEIGHT / 2.0, 0.2),
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
        font_size: FontSize::Px(40f32 / 1.2),
        ..default()
    };

    for (value, y) in [(score.current, 67f32), (score.highest, 18f32)] {
        commands.spawn((
            Text2d::new(value.to_string()),
            text_font.clone(),
            TextLayout::justify(Justify::Center),
            Transform::from_xyz(63.8f32, y, 0.3),
            ScoreboardScore,
        ));
    }
}

fn record(
    audio_assets: Res<AudioAssets>,
    mut audio_event: MessageWriter<AudioEvent>,
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

            audio_event.write(AudioEvent::new(&audio_assets.score, false));
        }
    }
}
