automod::dir!(pub "src/game/score");

use bevy_asset_loader::asset_collection::AssetCollectionApp;
use components::*;

use bevy::{prelude::*, sprite::Anchor::TopCenter};
use flappybust::{despawn, BasicMath, Switcher};
use itertools::Itertools;

use crate::SCREEN_HEIGHT;

use self::resources::ScoreAssets;

use super::{
    audio::{events::AudioEvent, resources::AudioAssets},
    bird::components::Bird,
    game_over::events::ScoreboardDisplayed,
    pipe::components::Pipe,
    GameState,
};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_collection::<ScoreAssets>()
            .add_systems(OnEnter(GameState::Ready), despawn::<ScoreboardScore>)
            .add_systems(OnEnter(GameState::Over), despawn::<CurrentScore>)
            .add_systems(OnEnter(GameState::Playing), spawn_current_score)
            .add_systems(
                Update,
                (
                    (record, display_current_score).run_if(in_state(GameState::Playing)),
                    display_scoreboard_score.run_if(in_state(GameState::Over)),
                ),
            );
    }
}

fn spawn_current_score(
    mut commands: Commands,
    score_assets: Res<ScoreAssets>,
    prev_score: Res<Score>,
) {
    let score = Score {
        current: 0,
        ..*prev_score
    };

    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(0f32, SCREEN_HEIGHT.half(), 0.2),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        score.current.to_string(),
                        TextStyle {
                            font: score_assets.teko_bold.clone(),
                            font_size: 64f32,
                            ..default()
                        },
                    )
                    .with_justify(JustifyText::Center),
                    text_anchor: TopCenter,
                    ..default()
                },
                CurrentScore,
            ));
        });

    commands.insert_resource(score);
}

fn display_current_score(
    score: Res<Score>,
    mut current_score: Query<&mut Text, With<CurrentScore>>,
) {
    let mut current_score = current_score.single_mut();

    for section in &mut current_score.sections {
        section.value = score.current.to_string();
    }
}

fn display_scoreboard_score(
    mut commands: Commands,
    score: Res<Score>,
    score_assets: Res<ScoreAssets>,
    mut scoreboard_displayed: EventReader<ScoreboardDisplayed>,
) {
    if scoreboard_displayed.is_empty() {
        return;
    }

    let text_style = TextStyle {
        font: score_assets.teko_bold.clone(),
        font_size: 40f32,
        ..default()
    };

    let box_size = Vec2::new(68f32, 20f32);

    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(63.8f32, 67f32, 0.3),
            sprite: Sprite {
                color: Color::NONE,
                custom_size: Some(box_size),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                Text2dBundle {
                    text: Text::from_section(score.current.to_string(), text_style.clone())
                        .with_justify(JustifyText::Center),
                    ..default()
                },
                ScoreboardScore,
            ));
        });

    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(63.8f32, 18f32, 0.3),
            sprite: Sprite {
                color: Color::NONE,
                custom_size: Some(box_size),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                Text2dBundle {
                    text: Text::from_section(score.highest.to_string(), text_style)
                        .with_justify(JustifyText::Center),
                    ..default()
                },
                ScoreboardScore,
            ));
        });

    scoreboard_displayed.clear();
}

fn record(
    audio_assets: Res<AudioAssets>,
    mut audio_event: EventWriter<AudioEvent>,
    bird: Query<&Transform, With<Bird>>,
    mut pipe: Query<(&mut Pipe, &Transform)>,
    mut score: ResMut<Score>,
) {
    let bird_transform = bird.single();

    for ((mut pipe, pipe_transform), _) in pipe.iter_mut().tuples() {
        // Increase score each time the bird has passed the pipe.
        if bird_transform.translation.x + Bird::WIDTH.half() > pipe_transform.translation.x
            && !pipe.hidden
        {
            score.current += 1;
            score.highest = score.current.max(score.highest);

            audio_event.send(AudioEvent::new(&audio_assets.score, false));

            // Prevent the score from increasing twice on frame changing too fast.
            pipe.hidden.on();

            break;
        }
    }
}
