use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};

/// Frame rate that per-frame speeds are tuned for, and the rate of `FixedUpdate`.
pub const BASE_FPS: f32 = 60.0;

/// Time since last frame, measured in 60 FPS frames (`1.0` at 60 FPS, `0.5` at 120 FPS).
///
/// Multiply per-frame speeds by this so animations outside `FixedUpdate` run the same at any frame rate.
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

/// Translation in the simulation, which moves in `FixedUpdate` one 60 FPS frame per step.
///
/// `Transform` only shows it, interpolated between the last two steps so motion stays
/// smooth at any frame rate (Bevy's `physics_in_fixed_timestep` example).
#[derive(Component, Clone, Copy, Deref, DerefMut)]
#[component(on_add = Position::on_add)]
#[require(Transform, PreviousPosition)]
pub struct Position(pub Vec3);

impl Position {
    /// Start from here, not from where the defaults would interpolate in from.
    fn on_add(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
        let position = world.get::<Position>(entity).unwrap().0;
        world.get_mut::<PreviousPosition>(entity).unwrap().0 = position;
        world.get_mut::<Transform>(entity).unwrap().translation = position;
    }
}

/// `Position` at the end of the previous fixed step.
#[derive(Component, Default)]
pub struct PreviousPosition(Vec3);

/// Keep where each entity was before this fixed step moves it.
pub fn save_previous_position(mut positions: Query<(&Position, &mut PreviousPosition)>) {
    for (position, mut previous) in &mut positions {
        previous.0 = position.0;
    }
}

/// Show each entity between its last two fixed steps, by how far time is into the next one.
pub fn interpolate_position(
    time: Res<Time<Fixed>>,
    mut positions: Query<(&Position, &PreviousPosition, &mut Transform)>,
) {
    let alpha = time.overstep_fraction();

    for (position, previous, mut transform) in &mut positions {
        transform.translation = previous.0.lerp(position.0, alpha);
    }
}

/// Endless horizontal scrolling for a pair of tiles placed `period` apart.
#[derive(Component)]
pub struct Scroll {
    /// Pixels per fixed step.
    pub speed: f32,
    pub period: f32,
}

/// Move scrolling tiles left, jumping each one back behind the other once it leaves.
pub fn scroll(mut tiles: Query<(&Scroll, &mut Position, &mut PreviousPosition)>) {
    for (scroll, mut position, mut previous) in &mut tiles {
        position.x -= scroll.speed;

        if position.x <= -scroll.period {
            position.x += 2.0 * scroll.period;
            // Jump without interpolating across the screen.
            previous.0.x += 2.0 * scroll.period;
        }
    }
}
