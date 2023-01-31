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

#[derive(Clone, Copy, PartialEq)]
enum Rank {
    Unit,
    Ten,
    Hunred,
}

#[derive(Component)]
struct ScoreRank(Rank);

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct HighScoreText;

#[derive(Resource, Clone, Default)]
pub struct Score {
    pub current: usize,
    pub highest: usize,
    textures: Vec<Handle<Image>>,
}

impl Score {
    pub const WIDTH: f32 = 24.;
    pub const HEIGHT: f32 = 36.;
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Playing, spawn)
            .add_system(record.run_in_state(GameState::Playing));
    }
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // score_board: Query<&Transform, With<Scoreboard>>,
) {
    let score = Score {
        textures: (0..10)
            .map(|i| asset_server.load(format!("images/{i}.png")))
            .collect(),
        ..default()
    };

    commands.insert_resource(score);

    commands.spawn_batch(vec![
        (SpriteBundle::default(), ScoreRank(Rank::Hunred)),
        (SpriteBundle::default(), ScoreRank(Rank::Ten)),
        (SpriteBundle::default(), ScoreRank(Rank::Unit)),
    ]);

    // let score_board = score_board.single();
    // let text = Text::from_section(
    //     "",
    //     TextStyle {
    //         font: asset_server.load("fonts/Teko-Bold.ttf"),
    //         font_size: 45.0,
    //         color: Color::WHITE,
    //     },
    // );

    // commands.spawn((
    //     Text2dBundle {
    //         text: text.clone(),
    //         transform: Transform::from_xyz(
    //             score_board.translation.x + 58.,
    //             score_board.translation.y + 30.,
    //             0.15,
    //         ),
    //         ..default()
    //     },
    //     ScoreText,
    // ));

    // commands.spawn((
    //     Text2dBundle {
    //         text,
    //         transform: Transform::from_xyz(
    //             score_board.translation.x + 58.,
    //             score_board.translation.y - 18.,
    //             0.15,
    //         ),
    //         ..default()
    //     },
    //     HighScoreText,
    // ));
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
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
            // let mut score_text = score_text.single_mut();
            // score_text.sections[0].value = score.current.to_string();

            // let mut best_score_text = hight_score_text.single_mut();
            // best_score_text.sections[0].value = score.highest.to_string();
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
