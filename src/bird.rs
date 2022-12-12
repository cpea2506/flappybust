use crate::{base::Base, pipe::Pipe, GameState};
use bevy::{math::Vec3, prelude::*};
use bevy_kira_audio::prelude::*;
use flappybust::{BooleanSwitcher, Math};
use itertools::Itertools;
use iyes_loopless::state::{CurrentState, NextState};
use rand::{
    distributions::{Distribution, Standard},
    random,
};

#[derive(Clone, Copy)]
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

impl BirdColor {
    pub fn raw_value(self) -> &'static str {
        match self {
            BirdColor::Red => "red",
            BirdColor::Blue => "blue",
            BirdColor::Yellow => "yellow",
        }
    }
}

pub type AnimationFrames = [Handle<Image>; 3];

#[derive(Component)]
pub struct FlapAnimation {
    timer: Timer,
    frames: AnimationFrames,
    current_frame: usize,
}

#[derive(Component, Default)]
pub struct PlayedAudio {
    die: bool,
    swoosh: bool,
    wing: bool,
}

fn load_animation_frames(asset_server: &AssetServer, bird_color: BirdColor) -> AnimationFrames {
    ["up", "mid", "down"].map(|state| {
        asset_server.load(format!(
            "images/bird_{color}_{state}.png",
            color = bird_color.raw_value()
        ))
    })
}

#[derive(Component, Clone, Copy)]
pub struct Bird {
    translation: Vec3,
    speed: f32,
    gravity: f32,
    jump: f32,
}

impl Bird {
    pub const WIDTH: f32 = 34.;
    pub const HEIGHT: f32 = 24.;

    fn new(x: f32, y: f32) -> Self {
        Bird {
            translation: Vec3::new(x, y, 0.2),
            speed: 0.,
            gravity: 0.098, // 9.8 m/s^2
            jump: -2.35,
        }
    }

    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        let bird_color = random::<BirdColor>();
        let animation_frames = load_animation_frames(&asset_server, bird_color);
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
            PlayedAudio::default(),
        ));
    }

    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    pub fn fly(
        mut commands: Commands,
        keys: Res<Input<KeyCode>>,
        buttons: Res<Input<MouseButton>>,
        asset_server: Res<AssetServer>,
        audio: Res<Audio>,
        game_state: Res<CurrentState<GameState>>,
        pipe: Query<&Transform, (With<Pipe>, Without<Bird>)>,
        base: Query<&Transform, (With<Base>, Without<Bird>, Without<Pipe>)>,
        mut bird: Query<(&mut Bird, &mut PlayedAudio, &mut Transform)>,
    ) {
        let (mut bird, mut played_audio, mut bird_transform) = bird.single_mut();
        let base_transform = base.iter().next().expect("base must be initialized first");

        let front_bird = bird_transform.translation.x + Bird::WIDTH.half();
        let bird_tail = bird_transform.translation.x - Bird::WIDTH.half();
        let bottom_bird = bird_transform.translation.y - Bird::HEIGHT.half();
        let bird_head = bird_transform.translation.y + Bird::HEIGHT.half();

        // collapsed with base
        let base_collapsed_position = Base::HEIGHT.half() + base_transform.translation.y;

        bird.speed += bird.gravity;

        if game_state.0 == GameState::Playing {
            if keys.pressed(KeyCode::Space) || buttons.just_pressed(MouseButton::Left) {
                if !played_audio.wing {
                    audio.play(asset_server.load("sounds/wing.wav"));
                    played_audio.wing.on();
                }

                bird.speed = bird.jump;
                played_audio.swoosh.off();
            }

            // collapsed with pipe
            for (pipe_transform, flipped_pipe_transform) in pipe.iter().tuples() {
                if bird_tail <= pipe_transform.translation.x + Pipe::WIDTH.half()
                    && front_bird >= pipe_transform.translation.x - Pipe::WIDTH.half()
                    && (bottom_bird <= pipe_transform.translation.y + Pipe::HEIGHT.half()
                        || bird_head >= flipped_pipe_transform.translation.y - Pipe::HEIGHT.half())
                {
                    audio.play(asset_server.load("sounds/hit.wav"));
                    commands.insert_resource(NextState(GameState::Over));
                    break;
                }
            }
        }

        bird_transform.translation.y -= bird.speed;

        // bird is doing free fall
        if bird.speed != bird.jump {
            if !played_audio.swoosh {
                audio.play(asset_server.load("sounds/swoosh.wav"));
                played_audio.swoosh.on();
            }

            played_audio.wing.off();
        }

        if bottom_bird <= base_collapsed_position {
            bird_transform.translation.y = base_collapsed_position + Bird::HEIGHT.half();

            if game_state.0 == GameState::Playing {
                commands.insert_resource(NextState(GameState::Over));
            }

            if !played_audio.die {
                audio.play(asset_server.load("sounds/die.wav"));
                played_audio.die.on();
            }
        }
    }

    pub fn flap(
        time: Res<Time>,
        mut bird: Query<(&mut FlapAnimation, &mut Handle<Image>), With<Bird>>,
    ) {
        let (mut animation, mut texture) = bird.single_mut();

        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            animation.current_frame = (animation.current_frame + 1) % 3;
            *texture = animation.frames[animation.current_frame].clone();
        }
    }
}
