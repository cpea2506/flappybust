use bevy::prelude::*;
use flappybust::{BooleanSwitcher, Math};
use itertools::Itertools;
use iyes_loopless::state::CurrentState;
use std::iter::successors;

use crate::{background::Background, bird::Bird, gameover::Scoreboard, pipe::Pipe, GameState};

#[derive(Clone, Copy, PartialEq)]
enum Rank {
    Unit,
    Ten,
    Hunred,
}

#[derive(Component)]
pub struct ScoreRank(Rank);

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct BestScoreText;

#[derive(Clone, Default)]
pub struct Score {
    pub value: usize,
    pub best_value: usize,
    textures: [Handle<Image>; 10],
}

impl Score {
    fn height() -> f32 {
        36.
    }

    fn width() -> f32 {
        24.
    }

    pub fn spawn(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        score_board: Query<&Transform, With<Scoreboard>>,
    ) {
        let mut score = Score::default();

        (0..10).for_each(|i| {
            let texture = asset_server.load(&format!("images/{i}.png"));
            score.textures[i] = texture;
        });

        commands.insert_resource(score);

        commands
            .spawn_bundle(SpriteBundle::default())
            .insert(ScoreRank(Rank::Hunred));

        commands
            .spawn_bundle(SpriteBundle::default())
            .insert(ScoreRank(Rank::Ten));

        commands
            .spawn_bundle(SpriteBundle::default())
            .insert(ScoreRank(Rank::Unit));

        let text = Text::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/Teko-Bold.ttf"),
                font_size: 45.0,
                color: Color::WHITE,
            },
        );

        let score_board = score_board.single();

        commands
            .spawn_bundle(Text2dBundle {
                text: text.clone(),
                transform: Transform::from_xyz(
                    score_board.translation.x + 58.,
                    score_board.translation.y + 30.,
                    0.3,
                ),
                ..default()
            })
            .insert(ScoreText);

        commands
            .spawn_bundle(Text2dBundle {
                text,
                transform: Transform::from_xyz(
                    score_board.translation.x + 58.,
                    score_board.translation.y - 18.,
                    0.3,
                ),
                ..default()
            })
            .insert(BestScoreText);
    }

    #[allow(clippy::type_complexity, clippy::too_many_arguments)]
    pub fn record(
        audio: Res<Audio>,
        asset_server: Res<AssetServer>,
        bird: Query<&Transform, With<Bird>>,
        game_state: Res<CurrentState<GameState>>,
        mut pipe: Query<(&mut Pipe, &Transform)>,
        mut score_rank: Query<
            (&ScoreRank, &mut Transform, &mut Handle<Image>),
            (Without<Pipe>, Without<Bird>),
        >,
        mut score_text: Query<&mut Text, With<ScoreText>>,
        mut best_score_text: Query<&mut Text, (With<BestScoreText>, Without<ScoreText>)>,
        mut score: ResMut<Score>,
    ) {
        let bird_transform = bird.single();

        for ((mut pipe, pipe_transform), _) in pipe.iter_mut().tuples() {
            // increase score each time the bird has passed the pipe
            if bird_transform.translation.x - Bird::width().half()
                > pipe_transform.translation.x + Pipe::width().half()
                && !pipe.has_passed
            {
                audio.play(asset_server.load("sounds/score.wav"));

                score.value += 1;
                score.best_value = score.value.max(score.best_value);

                // prevent the score from increasing twice
                // on frame changing too fast
                pipe.has_passed.on();

                break;
            }
        }

        if game_state.0 == GameState::Over {
            let mut score_text = score_text.single_mut();
            score_text.sections[0].value = score.value.to_string();

            let mut best_score_text = best_score_text.single_mut();
            best_score_text.sections[0].value = score.best_value.to_string();
        } else {
            // update texture base on score rank
            // count number of digit
            let digit_num = successors(Some(score.value), |&n| (n >= 10).then_some(n / 10)).count();
            let y_pos = Background::height().half() - 10. - Score::height().half();

            for (rank, mut transform, mut texture) in &mut score_rank {
                match (digit_num, rank.0) {
                    (1, Rank::Unit) => {
                        *texture = score.textures[score.value % 10].clone();
                        *transform = Transform::from_xyz(0., y_pos, 0.3);
                    }
                    (2, Rank::Unit) => {
                        *texture = score.textures[score.value % 10].clone();
                        *transform = Transform::from_xyz(Score::width().half(), y_pos, 0.3);
                    }
                    (2, Rank::Ten) => {
                        *texture = score.textures[score.value / 10].clone();
                        *transform = Transform::from_xyz(-Score::width().half(), y_pos, 0.3);
                    }
                    (3, Rank::Unit) => {
                        *texture = score.textures[score.value % 10].clone();
                        *transform = Transform::from_xyz(Score::width() + 1., y_pos, 0.3);
                    }
                    (3, Rank::Ten) => {
                        *texture = score.textures[score.value % 100 / 10].clone();
                        *transform = Transform::from_xyz(0., y_pos, 0.3);
                    }
                    (3, Rank::Hunred) => {
                        *texture = score.textures[score.value / 100].clone();
                        *transform = Transform::from_xyz(-Score::width() - 1., y_pos, 0.3);
                    }
                    _ => {}
                }
            }
        }
    }
}
