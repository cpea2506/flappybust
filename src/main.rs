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
            })
            .set(ImagePlugin::default_nearest()),
    )
    .init_state::<GameState>()
    .add_sub_state::<GameOverPhase>()
    .add_plugins(GamePlugin)
    .add_systems(Startup, setup_camera);

    #[cfg(target_arch = "wasm32")]
    app.add_systems(Last, smooth_frame_time);

    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Feed Bevy the average of recent frame times instead of the raw ones.
///
/// Browsers report frame times unevenly even at a steady frame rate,
/// which makes everything that moves by the frame time judder.
#[cfg(target_arch = "wasm32")]
fn smooth_frame_time(
    mut strategy: ResMut<bevy::time::TimeUpdateStrategy>,
    mut last: Local<Option<bevy::platform::time::Instant>>,
    mut average: Local<f32>,
) {
    /// Longer frames are real pauses (e.g. a hidden tab), passed through unaveraged.
    const MAX_SMOOTHED_SECS: f32 = 0.1;

    let now = bevy::platform::time::Instant::now();
    let Some(last) = last.replace(now) else {
        return;
    };
    let delta = (now - last).as_secs_f32();

    let delta = if delta > MAX_SMOOTHED_SECS {
        delta
    } else {
        *average = if *average == 0.0 {
            delta
        } else {
            average.lerp(delta, 0.1)
        };
        *average
    };

    *strategy =
        bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::from_secs_f32(delta));
}
