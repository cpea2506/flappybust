use bevy::prelude::*;
use rand::distributions::{Distribution, Standard};
use strum::AsRefStr;

#[derive(AsRefStr, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum BirdColor {
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

type AnimationFrames = [Handle<Image>; 3];

#[derive(Component)]
pub struct FlapAnimation {
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

#[derive(Component, Clone, Copy)]
pub struct Bird {
    pub size: Vec2,
    pub translation: Vec3,
    pub velocity: f32,
    pub gravity: f32,
    pub rotation: f32,
}

impl Bird {
    pub const WIDTH: f32 = 34.;
    pub const HEIGHT: f32 = 24.;

    pub fn new(x: f32, y: f32) -> Self {
        Bird {
            translation: Vec3::new(x, y, 0.3),
            velocity: 0.,
            size: Vec2::new(Self::WIDTH, Self::HEIGHT),
            gravity: 0.098, // 9.8 m/s^2
            rotation: 0.,
        }
    }
}

#[derive(Component)]
pub struct DeathBird {
    pub translation: Vec3,
}
