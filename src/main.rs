mod game;

use bevy::{asset::AssetMetaCheck, prelude::*};
use game::GamePlugin;

const SCREEN_WIDTH: f32 = 288f32;
const SCREEN_HEIGHT: f32 = 512f32;

/// Represents the current state of the game.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Ready,
    Playing,
    Over,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappybust 🦀🦋".to_string(),
                        resolution: (SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        // Needs the state schedules from `DefaultPlugins`, and must exist before `GamePlugin`.
        .init_state::<GameState>()
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Close the focused window on Escape (removed from Bevy in 0.14).
fn close_on_esc(
    mut commands: Commands,
    windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (entity, window) in &windows {
        if window.focused && input.just_pressed(KeyCode::Escape) {
            commands.entity(entity).despawn();
        }
    }
}
