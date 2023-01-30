use crate::constants::SCREEN_HEIGHT;

use super::base::Base;
use super::{audio::*, GameState};
use bevy::prelude::*;
use flappybust::Math;
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem};
use iyes_loopless::state::CurrentState;
use rand::distributions::{Distribution, Standard};
use rand::random;
use std::fmt::Display;

#[derive(Clone, Copy)]
enum BirdColor {
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

impl Display for BirdColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BirdColor::Red => "red",
            BirdColor::Blue => "blue",
            BirdColor::Yellow => "yellow",
        })
    }
}

type AnimationFrames = [Handle<Image>; 3];

#[derive(Component)]
struct FlapAnimation {
    timer: Timer,
    frames: AnimationFrames,
    current_frame: usize,
}

#[derive(Component, Clone, Copy)]
pub struct Bird {
    pub size: Vec2,

    translation: Vec3,
    velocity: f32,
    gravity: f32,
}

impl Bird {
    pub const WIDTH: f32 = 34.;
    pub const HEIGHT: f32 = 24.;

    fn new(x: f32, y: f32) -> Self {
        Bird {
            translation: Vec3::new(x, y, 0.3),
            velocity: 0.,
            size: Vec2::new(Self::WIDTH, Self::HEIGHT),
            gravity: 0.098, // 9.8 m/s^2
        }
    }
}

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Ready, spawn)
            .add_system(flap.run_not_in_state(GameState::Over))
            .add_system(fly.run_not_in_state(GameState::Ready));
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bird_color = random::<BirdColor>();
    let animation_frames = ["up", "mid", "down"]
        .map(|state| asset_server.load(format!("images/bird_{bird_color}_{state}.png")));
    let bird = Bird::new(-53., 9.);

    commands.spawn((
        SpriteBundle {
            texture: animation_frames[0].clone(), // 0. up, 1. mid, 2. down
            transform: Transform::from_translation(bird.translation),
            ..default()
        },
        bird,
        FlapAnimation {
            timer: Timer::from_seconds(0.15, TimerMode::Repeating),
            frames: animation_frames,
            current_frame: 0,
        },
    ));
}

fn fly(
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut bird: Query<(&mut Bird, &mut Transform)>,
    mut audio_event: EventWriter<AudioEvent>,
    audio_assets: Res<AudioAssets>,
) {
    let (mut bird, mut bird_transform) = bird.single_mut();

    if keys.just_pressed(KeyCode::Space) || buttons.just_pressed(MouseButton::Left) {
        audio_event.send(AudioEvent {
            audio: audio_assets.wing.clone(),
        });

        bird.velocity = -2.35
    } else {
        bird.velocity += bird.gravity;
        // audio_event.send(AudioEvent {
        //     audio: audio_assets.swoosh.clone(),
        // });
    }

    let base_collider_pos = Base::HEIGHT - SCREEN_HEIGHT.half();

    if bird_transform.translation.y - Bird::HEIGHT.half() <= base_collider_pos {
        bird_transform.translation.y = base_collider_pos + Bird::HEIGHT.half();

        return;
    }

    bird_transform.translation.y -= bird.velocity;
}

fn flap(time: Res<Time>, mut bird: Query<(&mut FlapAnimation, &mut Handle<Image>), With<Bird>>) {
    let (mut animation, mut texture) = bird.single_mut();

    animation.timer.tick(time.delta());

    if animation.timer.just_finished() {
        animation.current_frame = (animation.current_frame + 1) % 3;
        *texture = animation.frames[animation.current_frame].clone();
    }
}
