use bevy::prelude::*;

/// Frame rate that per-frame speeds are tuned for.
const BASE_FPS: f32 = 60.0;

/// Time since last frame, measured in 60 FPS frames (`1.0` at 60 FPS, `0.5` at 120 FPS).
///
/// Multiply per-frame speeds by this so gameplay runs the same at any frame rate.
pub fn frames(time: &Time) -> f32 {
    time.delta_secs() * BASE_FPS
}

/// Despawn entity that has specified components in the world.
pub fn despawn<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
