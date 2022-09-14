use bevy::{prelude::*, sprite::SpriteBundle};
use flappybust::Math;

use crate::{background::Background, bird::Bird, pipe::Pipe};

#[derive(Component, Default)]
pub struct Score {
    value: usize,
    numbers: [Handle<Image>; 10],
}

impl Score {
    fn height() -> f32 {
        36.
    }

    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        let mut score = Score::default();

        (0..10).for_each(|i| {
            let texture = asset_server.load(&format!("images/{i}.png"));
            score.numbers[i] = texture;
        });

        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(
                    0.,
                    Background::height().half() - 10. - Score::height().half(),
                    0.3,
                ),
                ..default()
            })
            .insert(score);
    }

    pub fn record(
        asset_server: Res<AssetServer>,
        audio: Res<Audio>,
        pipe: Query<&Transform, With<Pipe>>,
        bird: Query<&Transform, (With<Bird>, Without<Pipe>)>,
        mut score: Query<(&mut Score, &mut Handle<Image>), With<Score>>,
    ) {
        let bird_transform = bird.single();
        let (mut score, mut texture) = score.single_mut();

        *texture = score.numbers[score.value].clone();

        for pipe_transform in pipe.iter() {
            if bird_transform.translation.x - Bird::width().half()
                >= pipe_transform.translation.x + Pipe::width().half()
                && bird_transform.translation.x - Bird::width().half()
                    <= pipe_transform.translation.x + Pipe::width().half() + 0.5
            {
                audio.play(asset_server.load("sounds/score.wav"));

                score.value += 1;
            }
        }
    }
}
