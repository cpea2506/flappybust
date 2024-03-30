use super::date_time::DateTime;
use crate::{GameState, SCREEN_WIDTH};
use bevy::prelude::*;
use flappybust::ternary;

#[derive(Component, Default, Clone, Copy)]
struct Background {
    translation: Vec3,
    secondary: bool,
}

impl Background {
    fn new(x: f32, y: f32, secondary: bool) -> Self {
        Background {
            translation: Vec3 { x, y, z: 0f32 },
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
        app.add_systems(OnEnter(GameState::Ready), spawn)
            .add_systems(Update, moving.run_if(in_state(GameState::Playing)));
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
    for (mut background, mut transform) in &mut background {
        background.translation.x = (background.translation.x - 1.5f32) % SCREEN_WIDTH;

        transform.translation.x =
            background.translation.x + ternary!(background.secondary, SCREEN_WIDTH, 0f32);
    }
}
