use bevy::prelude::*;
use rand::random_range;

#[derive(Component)]
pub struct Pipe {
    /// Whether this is the upper (flipped) pipe of a pair.
    pub flip_y: bool,
    /// Whether the bird has already scored on this pipe.
    pub scored: bool,
}

impl Pipe {
    pub const WIDTH: f32 = 52f32;
    pub const HEIGHT: f32 = 320f32;
    const GAP: f32 = 80f32;
    /// Horizontal distance between two pipe pairs.
    pub(super) const SPACING: f32 = 175f32;

    /// Spawn a lower and an upper pipe at `x` with a random gap height.
    pub(super) fn spawn_pair(commands: &mut Commands, texture: &Handle<Image>, x: f32) {
        let y = random_range(-240f32..-50f32);

        for (flip_y, y) in [(false, y), (true, y + Self::GAP + Self::HEIGHT)] {
            commands.spawn((
                Sprite {
                    image: texture.clone(),
                    flip_y,
                    ..default()
                },
                Transform::from_xyz(x, y, 0.1),
                Pipe {
                    flip_y,
                    scored: false,
                },
            ));
        }
    }
}
