use bevy::{prelude::*, sprite::SpriteBundle};
use flappybust::{BooleanSwitcher, Math};
use itertools::Itertools;

use crate::{background::Background, bird::Bird, pipe::Pipe};

#[derive(Clone, Copy, Default)]
enum DigitRank {
    #[default]
    Unit,
    Ten,
    Hunred,
}

#[derive(Component, Default, Clone)]
pub struct Score {
    value: usize,
    numbers: [Handle<Image>; 10],
    rank: DigitRank,
}

impl Score {
    fn height() -> f32 {
        36.
    }

    fn width() -> f32 {
        24.
    }

    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        let mut score = Score {
            value: 189,
            ..Score::default()
        };

        let y_pos = Background::height().half() - 10. - Score::height().half();

        (0..10).for_each(|i| {
            let texture = asset_server.load(&format!("images/{i}.png"));
            score.numbers[i] = texture;
        });

        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(-Score::width() - 1., y_pos, 0.3),
                ..default()
            })
            .insert(Score {
                rank: DigitRank::Hunred,
                ..score.clone()
            });

        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(0., y_pos, 0.3),
                ..default()
            })
            .insert(Score {
                rank: DigitRank::Ten,
                ..score.clone()
            });

        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(Score::width() + 1., y_pos, 0.3),
                ..default()
            })
            .insert(score);
    }

    pub fn record(
        asset_server: Res<AssetServer>,
        audio: Res<Audio>,
        mut pipe: Query<(&mut Pipe, &Transform)>,
        mut score: Query<(&mut Score, &mut Handle<Image>)>,
        bird: Query<&Transform, With<Bird>>,
    ) {
        let bird_transform = bird.single();

        for (mut score, mut texture) in &mut score {
            for ((mut pipe, pipe_transform), _) in pipe.iter_mut().tuples() {
                // increase score each time the bird has passed the pipe
                if bird_transform.translation.x - Bird::width().half()
                    > pipe_transform.translation.x + Pipe::width().half()
                    && !pipe.has_passed
                {
                    score.value += 1;
                    audio.play(asset_server.load("sounds/score.wav"));

                    // prevent the score from increasing twice
                    // on frame change too fast
                    pipe.has_passed.on();
                }
            }

            // update texture base on score rank
            match score.rank {
                DigitRank::Hunred => *texture = score.numbers[score.value / 100].clone(),
                DigitRank::Ten => *texture = score.numbers[score.value % 100 / 10].clone(),
                DigitRank::Unit => *texture = score.numbers[score.value % 10].clone(),
            }
        }
    }
}
