use bevy::prelude::*;

pub trait BooleanSwitcher {
    fn on(&mut self);
    fn off(&mut self);
}

impl BooleanSwitcher for bool {
    fn off(&mut self) {
        *self = false
    }

    fn on(&mut self) {
        *self = true
    }
}

pub trait Math {
    fn half(self) -> f32;
}

impl Math for f32 {
    fn half(self) -> f32 {
        self / 2.
    }
}

/// This is to replace the cumbersome original short-hand if-else
#[macro_export]
macro_rules! ternary {
    ($condition:expr, $if:expr, $else:expr) => {
        if $condition {
            $if
        } else {
            $else
        }
    };
}

pub fn despawn_all(mut commands: Commands, entities: Query<Entity>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

pub fn despawn<T: Component>(mut commands: Commands, entity: Query<Entity, With<T>>) {
    commands.entity(entity.single()).despawn();
}
