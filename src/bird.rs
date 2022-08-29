use bevy::{
    math::{XY, XYZ},
    prelude::*,
};
use iyes_loopless::state::{CurrentState, NextState};
use rand::distributions::{Distribution, Standard};

use crate::{base::Base, GameState};

#[derive(Default)]
pub struct FlapTimer(f32);

#[derive(Component)]
pub struct Bird {
    translation: Vec3,
    color: BirdColor,
    speed: XY<f32>,
}

enum BirdColor {
    Red,
    Blue,
    Yellow,
}

impl BirdColor {
    fn raw_value(&self) -> &str {
        match self {
            BirdColor::Red => "red",
            BirdColor::Blue => "blue",
            BirdColor::Yellow => "yellow",
        }
    }
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
            translation: Vec3::new(-53., 9., 0.1),
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

    pub fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
        let bird_color = rand::random::<BirdColor>();
        let texture = asset_server.load(&format!(
            "images/{}bird-midflap.png",
            bird_color.raw_value()
        ));

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

    pub fn flap_system(
        mut commands: Commands,
        mut timer: ResMut<FlapTimer>,
        windows: Res<Windows>,
        asset_server: Res<AssetServer>,
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<(&mut Bird, &mut Handle<Image>, &mut Transform)>,
    ) {
        let window = windows.get_primary().unwrap();

        for (mut bird, mut texture, mut transform) in query.iter_mut() {
            let state = match timer.0 as usize % 3 {
                0 => "mid",
                1 => "up",
                _ => "down",
            };

            *texture = asset_server.load(&format!(
                "images/{}bird-{state}flap.png",
                bird.color.raw_value()
            ));

            transform.translation.y -= bird.speed.y;

            if keyboard_input.pressed(KeyCode::Space) {
                bird.speed.y = -2.5;
            } else {
                bird.speed.y += 0.1;
            }

            // change game state to over if collapsed with base
            if transform.translation.y < Base::height() - window.height() / 2. + Bird::height() / 2.
            {
                commands.insert_resource(NextState(GameState::Over));
            }
        }

        // timer.0 += 1;
    }
}
