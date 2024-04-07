use bevy::prelude::*;
use rand::distributions::{Distribution, Standard};

#[derive(Clone, Copy)]
pub(super) enum BirdColor {
    Red,
    Blue,
    Yellow,
}

impl Distribution<BirdColor> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> BirdColor {
        match rng.gen_range(0..3) {
            0 => BirdColor::Red,
            1 => BirdColor::Blue,
            _ => BirdColor::Yellow,
        }
    }
}

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
    pub velocity: f32,
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
    pub const DEFAULT_VELOCITY: f32 = -2.5;

    pub fn new(x: f32, y: f32) -> Self {
        Bird {
            translation: Vec3::new(x, y, 0.3),
            velocity: Self::DEFAULT_VELOCITY,
            gravity: 0.098,
            rotation: 25f32.to_radians(),
        }
    }
}

#[derive(Component)]
pub(super) struct BirdSoul {
    pub translation: Vec3,
}
