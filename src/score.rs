use bevy::{prelude::*, sprite::SpriteBundle};
use flappybust::{BooleanSwitcher, Math};
use itertools::Itertools;
use std::iter::successors;

use crate::{background::Background, bird::Bird, pipe::Pipe};

#[derive(Clone, Copy, PartialEq)]
enum Rank {
    Unit,
    Ten,
    Hunred,
}

#[derive(Component)]
pub struct ScoreRank(Rank);

#[derive(Clone, Default)]
pub struct Score {
    value: usize,
    textures: [Handle<Image>; 10],
}

impl Score {
    fn height() -> f32 {
        36.
    }

    fn width() -> f32 {
        24.
    }

    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    }

    #[allow(clippy::type_complexity)]
    pub fn record(
        asset_server: Res<AssetServer>,
        audio: Res<Audio>,
        mut pipe: Query<(&mut Pipe, &Transform)>,
        mut score_rank: Query<
            (&ScoreRank, &mut Handle<Image>, &mut Transform),
            (Without<Pipe>, Without<Bird>),
        >,
        mut score: ResMut<Score>,
        bird: Query<&Transform, With<Bird>>,
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

                // prevent the score from increasing twice
                // on frame changing too fast
                pipe.has_passed.on();

                break;
            }
        }

        // update texture base on score rank
        let digit_num = successors(Some(score.value), |&n| (n >= 10).then_some(n / 10)).count();
        let y_pos = Background::height().half() - 10. - Score::height().half();

        for (rank, mut texture, mut transform) in &mut score_rank {
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
