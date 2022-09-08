use crate::{base::Base, pipe::Pipe, GameState};
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

#[derive(Component)]
pub struct FlapAnimation {
    timer: Timer,
    frames: [Handle<Image>; 3],
    current_frame: usize,
}

#[derive(Component, Clone, Copy)]
pub struct Bird {
    translation: Vec3,
    speed: Vec2,
}

#[derive(Debug, Clone, Copy)]
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
    fn new(x: f32, y: f32) -> Self {
        Bird {
            translation: Vec3::new(x, y, 0.2),
            speed: default(),
        }
    }

    pub fn height() -> f32 {
        24.
    }

    pub fn width() -> f32 {
        34.
    }

    fn load_image(asset_server: &AssetServer, bird_color: BirdColor, state: &str) -> Handle<Image> {
        asset_server.load(&format!("images/{bird_color:?}bird-{state}flap.png"))
    }

    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        let bird_color = random::<BirdColor>();
        let mid_bird = Bird::load_image(&asset_server, bird_color, "mid");
        let up_bird = Bird::load_image(&asset_server, bird_color, "up");
        let down_bird = Bird::load_image(&asset_server, bird_color, "down");

        let bird = Bird::new(-53., 9.);

        commands
            .spawn_bundle(SpriteBundle {
                texture: mid_bird.clone(),
                transform: Transform::from_translation(bird.translation),
                ..default()
            })
            .insert(bird)
            .insert(FlapAnimation {
                timer: Timer::from_seconds(0.15, true),
                frames: [mid_bird, up_bird, down_bird],
                current_frame: 0,
            });
    }

    #[allow(clippy::type_complexity)]
    pub fn fly(
        mut commands: Commands,
        keys: Res<Input<KeyCode>>,
        buttons: Res<Input<MouseButton>>,
        game_state: Res<CurrentState<GameState>>,
        mut bird: Query<(&mut Bird, &mut Transform)>,
        pipe: Query<&Transform, (With<Pipe>, Without<Bird>)>,
        base: Query<&Transform, (With<Base>, Without<Bird>, Without<Pipe>)>,
    ) {
        let (mut bird, mut bird_transform) = bird.single_mut();
        let base_transform = base.iter().next().expect("base must be initialized first");

        bird.speed.y += 0.1;

        let front_bird = bird_transform.translation.x + Bird::width().half();
        let bird_tail = bird_transform.translation.x - Bird::width().half();
        let bottom_bird = bird_transform.translation.y - Bird::height().half();
        let bird_head = bird_transform.translation.y + Bird::height().half();

        if game_state.0 == GameState::Playing {
            if keys.pressed(KeyCode::Space) || buttons.just_pressed(MouseButton::Left) {
                bird.speed.y = -2.5;
            }

            // collapsed with pipe
            for (pipe_transform, flipped_pipe_transform) in pipe.iter().tuples() {
                if bird_tail <= pipe_transform.translation.x + Pipe::width().half()
                    && front_bird >= pipe_transform.translation.x - Pipe::width().half()
                    && (bottom_bird <= pipe_transform.translation.y + Pipe::height().half()
                        || bird_head
                            >= flipped_pipe_transform.translation.y - Pipe::height().half())
                {
                    commands.insert_resource(NextState(GameState::Over));
                    break;
                }
            }
        }

        bird_transform.translation.y -= bird.speed.y;

        // collapsed with base
        let base_collapsed_position = Base::height().half() + base_transform.translation.y;

        if bottom_bird <= base_collapsed_position {
            bird_transform.translation.y = base_collapsed_position + Bird::height().half();

            if game_state.0 == GameState::Playing {
                commands.insert_resource(NextState(GameState::Over));
            }
        }
    }

    pub fn flap(
        time: Res<Time>,
        mut bird_texture: Query<(&mut FlapAnimation, &mut Handle<Image>), With<Bird>>,
    ) {
        let (mut animation, mut texture) = bird_texture.single_mut();

        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            animation.current_frame = (animation.current_frame + 1) % 3;

            *texture = animation.frames[animation.current_frame].clone();
        }
    }
}