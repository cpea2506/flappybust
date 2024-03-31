use bevy::prelude::*;
use flappybust::{ternary, BasicMath};
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

use crate::{game::date_time::DateTime, SCREEN_WIDTH};

use super::resources::PipeAssets;

#[derive(Component, Default)]
pub struct Pipe {
    pub size: Vec2,
    pub hidden: bool,

    flip_y: bool,
    translation: Vec3,
}

impl Pipe {
    pub(super) const WIDTH: f32 = 52f32;
    pub(super) const HEIGHT: f32 = 320f32;
    const GAP: f32 = 80f32;

    fn new(x: f32, y: f32, flip_y: bool) -> Self {
        Pipe {
            translation: Vec3::new(x, y, 0.1),
            size: Vec2::new(Self::WIDTH, Self::HEIGHT),
            flip_y,
            ..default()
        }
    }

    fn generate_bundle(self, texture: &Handle<Image>) -> (SpriteBundle, Self) {
        (
            SpriteBundle {
                sprite: Sprite {
                    flip_y: self.flip_y,
                    ..default()
                },
                texture: texture.clone(),
                transform: Transform::from_translation(self.translation),
                ..default()
            },
            self,
        )
    }

    /// Generate number of pipes by `num_pipe`.
    #[inline]
    pub(super) fn genrate_self(
        num_pipe: u32,
        first_time: bool,
        commands: &mut Commands,
        pipe_assets: &Res<PipeAssets>,
        datetime: &Res<DateTime>,
    ) {
        let texture = match **datetime {
            DateTime::Day => pipe_assets.green.clone(),
            DateTime::Night => pipe_assets.red.clone(),
        };

        let mut rng = thread_rng();
        let y_between = Uniform::new(-240f32, -50f32);

        // Spawn first 2 pipes.
        (0..num_pipe).for_each(|i| {
            let pipe = Self::new(
                ternary!(first_time, SCREEN_WIDTH, SCREEN_WIDTH.half())
                    + Self::WIDTH.half()
                    + 175f32 * i as f32,
                y_between.sample(&mut rng),
                false,
            );
            let flipped_pipe = Self::new(
                pipe.translation.x,
                pipe.translation.y + Self::GAP + Self::HEIGHT,
                true,
            );

            commands.spawn_batch(vec![
                pipe.generate_bundle(&texture),
                flipped_pipe.generate_bundle(&texture),
            ]);
        });
    }
}
