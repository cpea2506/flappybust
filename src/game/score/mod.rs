pub mod components;
use components::*;

use crate::constants::SCREEN_HEIGHT;

use bevy::prelude::*;
use flappybust::{BooleanSwitcher, Math};
use itertools::Itertools;
use iyes_loopless::{
    prelude::{AppLooplessStateExt, IntoConditionalSystem},
    state::CurrentState,
};
use std::iter::successors;

use super::{
    audio::{AudioAssets, AudioEvent},
    bird::components::Bird,
    pipe::Pipe,
    GameState,
};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_enter_system(GameState::Playing, playing_score_spawn)
            .add_enter_system(GameState::Over, over_score_spawn)
            .add_system(record.run_not_in_state(GameState::Ready));
    }
}

fn playing_score_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    prev_score: Res<Score>,
) {
    let score = Score {
        textures: (0..10)
            .map(|i| asset_server.load(format!("images/{i}.png")))
            .collect(),
        current: 0,
        ..*prev_score
    };

    commands.insert_resource(score);

    commands.spawn_batch(vec![
        (SpriteBundle::default(), ScoreRank(Rank::Hunred)),
        (SpriteBundle::default(), ScoreRank(Rank::Ten)),
        (SpriteBundle::default(), ScoreRank(Rank::Unit)),
    ]);
}

/// score display in scoreboard in Over state
fn over_score_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text = Text::from_section(
        "",
        TextStyle {
            font: asset_server.load("fonts/Teko-Bold.ttf"),
            font_size: 45.0,
            color: Color::WHITE,
        },
    );

    commands.spawn((
        Text2dBundle {
            text: text.clone(),
            visibility: Visibility::INVISIBLE,
            transform: Transform::from_xyz(58., 87., 0.3),
            ..default()
        },
        ScoreText,
    ));

    commands.spawn((
        Text2dBundle {
            text,
            visibility: Visibility::INVISIBLE,
            transform: Transform::from_xyz(58., 39., 0.3),
            ..default()
        },
        HighScoreText,
    ));
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn record(
    audio_assets: Res<AudioAssets>,
    mut audio_event: EventWriter<AudioEvent>,
    bird: Query<&Transform, With<Bird>>,
    game_state: Res<CurrentState<GameState>>,
    mut pipe: Query<(&mut Pipe, &Transform)>,
    mut score_rank: Query<
        (&ScoreRank, &mut Transform, &mut Handle<Image>),
        (Without<Pipe>, Without<Bird>),
    >,
    mut score_text: Query<&mut Text, With<ScoreText>>,
    mut hight_score_text: Query<&mut Text, (With<HighScoreText>, Without<ScoreText>)>,
    mut score: ResMut<Score>,
) {
    let bird_transform = bird.single();

    for ((mut pipe, pipe_transform), _) in pipe.iter_mut().tuples() {
        // increase score each time the bird has passed the pipe
        if bird_transform.translation.x + Bird::WIDTH.half() > pipe_transform.translation.x
            && !pipe.hidden
        {
            audio_event.send(AudioEvent::new(&audio_assets.score, false));

            score.current += 1;
            score.highest = score.current.max(score.highest);

            // prevent the score from increasing twice
            // on frame changing too fast
            pipe.hidden.on();

            break;
        }
    }

    match game_state.0 {
        GameState::Over => {
            let mut score_text = score_text.single_mut();
            score_text.sections[0].value = score.current.to_string();

            let mut best_score_text = hight_score_text.single_mut();
            best_score_text.sections[0].value = score.highest.to_string();
        }
        _ => {
            // update texture base on score rank
            // count number of digit
            let digit_num =
                successors(Some(score.current), |&n| (n >= 10).then_some(n / 10)).count();
            let y_pos = SCREEN_HEIGHT.half() - 10. - Score::HEIGHT.half();

            for (rank, mut transform, mut texture) in &mut score_rank {
                match (digit_num, rank.0) {
                    (1, Rank::Unit) => {
                        *texture = score.textures[score.current % 10].clone();
                        *transform = Transform::from_xyz(0., y_pos, 0.3);
                    }
                    (2, Rank::Unit) => {
                        *texture = score.textures[score.current % 10].clone();
                        *transform = Transform::from_xyz(Score::WIDTH.half(), y_pos, 0.3);
                    }
                    (2, Rank::Ten) => {
                        *texture = score.textures[score.current / 10].clone();
                        *transform = Transform::from_xyz(-Score::WIDTH.half(), y_pos, 0.3);
                    }
                    (3, Rank::Unit) => {
                        *texture = score.textures[score.current % 10].clone();
                        *transform = Transform::from_xyz(Score::WIDTH + 1., y_pos, 0.3);
                    }
                    (3, Rank::Ten) => {
                        *texture = score.textures[score.current % 100 / 10].clone();
                        *transform = Transform::from_xyz(0., y_pos, 0.3);
                    }
                    (3, Rank::Hunred) => {
                        *texture = score.textures[score.current / 100].clone();
                        *transform = Transform::from_xyz(-Score::WIDTH - 1., y_pos, 0.3);
                    }
                    _ => {}
                }
            }
        }
    }
}
