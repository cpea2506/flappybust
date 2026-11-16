use crate::{
    game::{DateTime, ImageAssets},
    SCREEN_WIDTH,
};
use bevy::prelude::*;
use flappybust::{ternary, BasicMath};
use rand::random_range;

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

    fn generate_bundle(self, texture: &Handle<Image>) -> (Sprite, Transform, Self) {
        (
            Sprite {
                image: texture.clone(),
                flip_y: self.flip_y,
                ..default()
            },
            Transform::from_translation(self.translation),
            self,
        )
    }

    /// Generate number of pipes by `num_pipe`.
    #[inline]
    pub(super) fn genrate_self(
        num_pipe: u32,
        first_time: bool,
        commands: &mut Commands,
        image_assets: &Res<ImageAssets>,
        datetime: &Res<DateTime>,
    ) {
        let texture = match **datetime {
            DateTime::Day => image_assets.green_pipe.clone(),
            DateTime::Night => image_assets.red_pipe.clone(),
        };

        // Spawn first 2 pipes.
        (0..num_pipe).for_each(|i| {
            let pipe = Self::new(
                ternary!(first_time, SCREEN_WIDTH, SCREEN_WIDTH.half())
                    + Self::WIDTH.half()
                    + 175f32 * i as f32,
                random_range(-240f32..-50f32),
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
