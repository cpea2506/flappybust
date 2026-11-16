mod game;

use bevy::{asset::AssetMetaCheck, prelude::*};
use game::GamePlugin;

const SCREEN_WIDTH: f32 = 288f32;
const SCREEN_HEIGHT: f32 = 512f32;
/// Horizontal scroll speed of the background, base and pipes, in pixels per 60 FPS frame.
const GAME_SPEED: f32 = 2.0;

/// Represents the current state of the game.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Ready,
    Playing,
    Over,
}

/// Steps of the game over screen, in order.
#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[source(GameState = GameState::Over)]
pub enum GameOverPhase {
    /// Game over text drops in and the scoreboard rises.
    #[default]
    Board,
    /// Scores appear and the medal, if earned, shrinks into place.
    Medal,
    /// The bird's soul flies up to heaven.
    Soul,
    /// The restart button waits for input.
    Restart,
}

/// From the first flap until restarting, whether the bird is alive or dead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InRound;

impl ComputedStates for InRound {
    type SourceStates = GameState;

    fn compute(state: GameState) -> Option<Self> {
        matches!(state, GameState::Playing | GameState::Over).then_some(InRound)
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Flappybust 🦀🦋".to_string(),
                    resolution: (SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
    )
    .init_state::<GameState>()
    .add_computed_state::<InRound>()
    .add_sub_state::<GameOverPhase>()
    .add_plugins(GamePlugin)
    .add_systems(Startup, setup_camera);

    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Update, close_on_esc);

    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Close the focused window on Escape.
///
/// Not on the web, where closing the window only freezes the game on the page.
#[cfg(not(target_arch = "wasm32"))]
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
