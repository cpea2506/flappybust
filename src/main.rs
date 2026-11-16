mod game;

use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    window::{PrimaryWindow, close_on_esc},
};
use game::{GamePlugin, game_over::events::RestartButtonDisplayed};

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
    let mut app = App::new();

    app.init_state::<GameState>()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Flappybust 🦀🦋".to_string(),
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    ..default()
                }),
                ..default()
            }),
            GamePlugin,
        ))
        .add_systems(Startup, setup_camera)
        .add_systems(
            Update,
            (
                start_game.run_if(in_state(GameState::Ready)),
                restart_game.run_if(in_state(GameState::Over)),
                close_on_esc,
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn start_game(
    key: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if key.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
        next_state.set(GameState::Playing);
    }
}

fn restart_game(
    key: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut next_state: ResMut<NextState<GameState>>,
    window: Query<&Window, With<PrimaryWindow>>,
    restart_btn_displayed: EventReader<RestartButtonDisplayed>,
) {
    if restart_btn_displayed.is_empty() {
        return;
    }

    if key.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Ready);
    }

    if mouse.just_pressed(MouseButton::Left)
        && let Some(cursor) = window.single().cursor_position()
        // The cursor must be inside restart btn area.
        && cursor.y > 273f32
        && cursor.y < 310f32
        && cursor.x > 90f32
        && cursor.x < 200f32
    {
        next_state.set(GameState::Ready);
    }
}
