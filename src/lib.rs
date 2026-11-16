use bevy::prelude::*;

/// Frame rate that per-frame speeds are tuned for.
const BASE_FPS: f32 = 60.0;

/// Time since last frame, measured in 60 FPS frames (`1.0` at 60 FPS, `0.5` at 120 FPS).
///
/// Multiply per-frame speeds by this so gameplay runs the same at any frame rate.
pub fn frames(time: &Time) -> f32 {
    time.delta_secs() * BASE_FPS
}

/// A one-shot sound that despawns once finished.
pub fn sound(source: &Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer::new(source.clone()), PlaybackSettings::DESPAWN)
}

/// Whether the player flapped this frame: Space, a left click or a touch.
pub fn flap_pressed(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
) -> bool {
    keys.just_pressed(KeyCode::Space)
        || mouse.just_pressed(MouseButton::Left)
        || touches.any_just_pressed()
}

/// Endless horizontal scrolling for a pair of tiles placed `period` apart.
#[derive(Component)]
pub struct Scroll {
    /// Pixels per 60 FPS frame.
    pub speed: f32,
    pub period: f32,
}

/// Move scrolling tiles left, jumping each one back behind the other once it leaves.
pub fn scroll(time: Res<Time>, mut tiles: Query<(&Scroll, &mut Transform)>) {
    for (scroll, mut transform) in &mut tiles {
        transform.translation.x -= scroll.speed * frames(&time);

        if transform.translation.x <= -scroll.period {
            transform.translation.x += 2.0 * scroll.period;
        }
    }
}
