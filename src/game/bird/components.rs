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

#[derive(Component, Clone, Copy)]
pub struct Bird {
    pub translation: Vec3,
    pub velocity: f32,
    pub gravity: f32,
    pub rotation: f32,
}

impl Bird {
    pub const WIDTH: f32 = 34f32;
    pub const HEIGHT: f32 = 24f32;
    // Original Flappy Bird physics (as recreated by FlapPyBird at 30 FPS), in 60 FPS frames.
    pub const DEFAULT_VELOCITY: f32 = -4.5;
    pub const MAX_VELOCITY: f32 = 5.0;

    pub fn new(x: f32, y: f32) -> Self {
        Bird {
            translation: Vec3::new(x, y, 0.3),
            velocity: Self::DEFAULT_VELOCITY,
            gravity: 0.25,
            rotation: 25f32.to_radians(),
        }
    }
}

#[derive(Component)]
pub(super) struct BirdSoul;
