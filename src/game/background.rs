use crate::{constants::SCREEN_WIDTH, GameState};
use bevy::prelude::*;
use flappybust::ternary;
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem};

use super::resources::DateTime;

#[derive(Component, Default, Clone, Copy)]
struct Background {
    translation: Vec3,
    secondary: bool,
}

impl Background {
    fn new(x: f32, y: f32, secondary: bool) -> Self {
        Background {
            translation: Vec3 { x, y, z: 0. },
            secondary,
        }
    }

    fn generate_bundle(self, texture: &Handle<Image>) -> (SpriteBundle, Self) {
        (
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_translation(self.translation),
                ..default()
            },
            self,
        )
    }
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Ready, spawn)
            .add_system(moving.run_in_state(GameState::Playing));
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, datetime: Res<DateTime>) {
    let background = Background::default();
    let secondary_background =
        Background::new(background.translation.x, background.translation.y, true);
    let texture = asset_server.load(format!("images/bg_{}.png", (*datetime).as_ref()));

    commands.spawn_batch(vec![
        background.generate_bundle(&texture),
        secondary_background.generate_bundle(&texture),
    ]);
}

fn moving(mut background: Query<(&mut Background, &mut Transform)>) {
    let screen_width = SCREEN_WIDTH;

    for (mut background, mut transform) in &mut background {
        background.translation.x = (background.translation.x - 1.) % screen_width;

        transform.translation.x =
            background.translation.x + ternary!(background.secondary, SCREEN_WIDTH, 0.);
    }
}
