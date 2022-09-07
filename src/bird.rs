use bevy::{
    math::{Vec2, Vec3},
    prelude::*,
};
use flappybust::Math;
use itertools::Itertools;
use iyes_loopless::state::{CurrentState, NextState};
use rand::{
    distributions::{Distribution, Standard},
    random,
};

use crate::{base::Base, pipe::Pipe, GameState};

#[derive(Default)]
pub struct FlapTimer(f32);

#[derive(Component)]
pub struct Bird {
    translation: Vec3,
    color: BirdColor,
    speed: Vec2,
}

#[derive(Debug)]
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

impl Bird {
    fn new(color: BirdColor) -> Self {
        Bird {
            translation: Vec3::new(-53., 9., 0.2),
            color,
            speed: default(),
        }
    }

    pub fn height() -> f32 {
        24.
    }

    pub fn width() -> f32 {
        34.
    }

    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        let bird_color = random::<BirdColor>();
        let texture = asset_server.load(&format!("images/{:?}bird-midflap.png", bird_color));
        let bird = Bird::new(bird_color);

        commands
            .spawn_bundle(SpriteBundle {
                texture,
                transform: Transform::from_translation(bird.translation),
                ..default()
            })
            .insert(bird);

        commands.insert_resource(FlapTimer::default());
    }

    pub fn flap(
        mut commands: Commands,
        mut timer: ResMut<FlapTimer>,
        asset_server: Res<AssetServer>,
        keyboard_input: Res<Input<KeyCode>>,
        windows: Res<Windows>,
        game_state: Res<CurrentState<GameState>>,
        mut bird: Query<(&mut Bird, &mut Handle<Image>, &mut Transform)>,
        pipe: Query<&Transform, (With<Pipe>, Without<Bird>)>,
    ) {
        let window = windows.get_primary().unwrap();
        let (mut bird, mut texture, mut bird_transform) = bird.single_mut();
        // let state = match timer.0 as usize % 3 {
        //     0 => "mid",
        //     1 => "up",
        //     _ => "down",
        // };

        // *texture = asset_server.load(&format!("images/{:?}bird-{state}flap.png", bird.color));
        bird.speed.y += 0.1;

        if game_state.0 == GameState::Playing {
            if keyboard_input.pressed(KeyCode::Space) {
                bird.speed.y = -2.5;
            }

            // collapsed with pipe
            for (pipe_transform, flipped_pipe_transform) in pipe.iter().tuples() {
                if bird_transform.translation.x + Bird::width().half()
                    >= pipe_transform.translation.x - Pipe::width().half()
                    && bird_transform.translation.x - Bird::width().half()
                        <= pipe_transform.translation.x + Pipe::width().half()
                {
                    if bird_transform.translation.y - Bird::height().half()
                        <= pipe_transform.translation.y + Pipe::height().half()
                        || bird_transform.translation.y + Bird::height().half()
                            >= flipped_pipe_transform.translation.y - Pipe::height().half()
                    {
                        commands.insert_resource(NextState(GameState::Over));
                        break;
                    }
                }
            }
        }

        bird_transform.translation.y -= bird.speed.y;

        // collapsed with base
        let base_collapsed_position =
            Base::height() + Bird::height().half() - window.height().half();

        if bird_transform.translation.y <= base_collapsed_position {
            bird_transform.translation.y = base_collapsed_position;

            if game_state.0 == GameState::Playing {
                commands.insert_resource(NextState(GameState::Over));
            }
        }
    }
}
