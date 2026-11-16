mod game;

use bevy::{
    asset::AssetMetaCheck,
    platform::time::Instant,
    prelude::*,
    time::{TimeSystems, TimeUpdateStrategy},
    window::{Monitor, PrimaryMonitor},
};
use game::GamePlugin;
use std::time::Duration;

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

fn main() {
    let mut app = App::new();

    app.add_plugins(
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
    .init_state::<GameState>()
    .add_plugins(GamePlugin)
    .add_systems(Startup, setup_camera)
    .add_systems(First, snap_frame_time.before(TimeSystems));

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

/// Advance game time in whole display refreshes instead of the measured frame time.
///
/// The measured time jitters by a millisecond or more while frames reach the screen at a
/// steady rate, which made moving sprites (the bird most of all) step unevenly.
fn snap_frame_time(
    monitor: Query<&Monitor, With<PrimaryMonitor>>,
    mut strategy: ResMut<TimeUpdateStrategy>,
    mut last: Local<Option<Instant>>,
    mut estimate: Local<f64>,
) {
    let now = Instant::now();
    let Some(prev) = last.replace(now) else {
        return;
    };
    let elapsed = (now - prev).as_secs_f64();

    let refresh = match monitor
        .iter()
        .next()
        .and_then(|m| m.refresh_rate_millihertz)
    {
        Some(millihertz) => 1000.0 / millihertz as f64,
        // Browsers don't report a monitor, so estimate the refresh period instead.
        None => {
            *estimate = estimate_refresh(*estimate, elapsed);
            *estimate
        }
    };

    let refreshes = (elapsed / refresh).round().max(1.0);

    *strategy = TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(refresh * refreshes));
}

/// Refine the estimated refresh period (in seconds) with a newly measured frame time.
///
/// A faster frame means the estimate was too long (e.g. the first frame included loading),
/// so take it outright. Otherwise smooth towards the frame time divided by the refreshes it
/// spanned, so dropped frames don't pull the estimate up.
fn estimate_refresh(estimate: f64, elapsed: f64) -> f64 {
    if estimate == 0.0 || elapsed < estimate * 0.8 {
        return elapsed;
    }

    let refreshes = (elapsed / estimate).round();

    estimate + (elapsed / refreshes - estimate) * 0.05
}

#[cfg(test)]
mod tests {
    use super::estimate_refresh;

    #[test]
    fn estimate_refresh_settles_on_jittery_60hz() {
        // A slow first frame from loading, then jittery 60 Hz frames with a dropped one.
        let mut estimate = estimate_refresh(0.0, 0.25);
        for (i, jitter) in [0.0012, -0.0009, 0.0004, -0.0013, 0.0008]
            .iter()
            .cycle()
            .take(300)
            .enumerate()
        {
            let dropped = if i == 150 { 2.0 } else { 1.0 };
            estimate = estimate_refresh(estimate, 1.0 / 60.0 * dropped + jitter);
        }

        assert!(
            (estimate - 1.0 / 60.0).abs() < 0.0003,
            "estimate was {estimate}"
        );
    }
}
