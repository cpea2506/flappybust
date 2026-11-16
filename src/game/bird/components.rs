use super::super::base::components::Base;
use bevy::prelude::*;

type AnimationFrames = Vec<Handle<Image>>;

#[derive(Component)]
pub(super) struct FlapAnimation {
    pub timer: Timer,
    pub frames: AnimationFrames,
    pub current_frame: usize,
}

impl FlapAnimation {
    pub fn new(seconds: f32, frames: AnimationFrames) -> Self {
        FlapAnimation {
            frames,
            current_frame: 0,
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
        }
    }
}

#[derive(Component, Default)]
pub(super) struct BouncingAnimation {
    pub phase: f32,
}

impl BouncingAnimation {
    /// Phase step per frame; also sets the bounce height.
    pub const SPEED: f32 = 0.16;
}

#[derive(Component)]
pub struct Bird {
    pub velocity: f32,
    pub rotation: f32,
}

impl Bird {
    pub const WIDTH: f32 = 34f32;
    pub const HEIGHT: f32 = 24f32;
    // Original Flappy Bird physics (as recreated by FlapPyBird at 30 FPS), in 60 FPS frames.
    pub const DEFAULT_VELOCITY: f32 = -4.5;
    pub const MAX_VELOCITY: f32 = 5.0;
    pub const GRAVITY: f32 = 0.25;

    pub const START: Vec2 = Vec2::new(-53f32, 9f32);
    /// Height of a bird lying on the base, dead.
    pub const GROUND_Y: f32 = Base::TOP + Self::HEIGHT / 2.0;
    pub const FLAP_FRAME_SECS: f32 = 0.12;
    /// Nose-up angle after a flap, and the highest it can point.
    pub const MAX_ROTATION: f32 = 25f32.to_radians();
    /// Straight down, the lowest it can point.
    pub const MIN_ROTATION: f32 = -90f32.to_radians();
    /// Radians turned nose-down per frame.
    pub const ROTATION_SPEED: f32 = 1f32 / 40f32;
}

impl Default for Bird {
    fn default() -> Self {
        Bird {
            velocity: Self::DEFAULT_VELOCITY,
            rotation: Self::MAX_ROTATION,
        }
    }
}

#[derive(Component)]
pub(super) struct BirdSoul;

impl BirdSoul {
    /// Height at which the soul has left the screen.
    pub const HEAVEN_Y: f32 = 267f32;
}
