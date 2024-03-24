#![feature(decl_macro)]

use bevy::prelude::*;

/// Add useful helper methods for Boolean type.
pub trait BooleanSwitcher {
    /// Change Boolean value to true.
    fn on(&mut self);
    /// Change Boolean value to false.
    fn off(&mut self);
    /// Toggle a Boolean value from `true` to `false` or from `false` to `true`.
    fn toggle(&mut self);
}

impl BooleanSwitcher for bool {
    fn off(&mut self) {
        *self = false
    }

    fn on(&mut self) {
        *self = true
    }

    fn toggle(&mut self) {
        *self = !*self;
    }
}

/// Add useful basic math operations.
pub trait BasicMath {
    /// Divide any fractional number by two.
    fn half(self) -> Self;
}

impl BasicMath for f32 {
    fn half(self) -> f32 {
        self / 2.0
    }
}

impl BasicMath for Vec2 {
    fn half(self) -> Vec2 {
        self / 2.0
    }
}

/// Replace the cumbersome original short-hand if-else
pub macro ternary($condition:expr, $then:expr, $else:expr) {
    if $condition {
        $then
    } else {
        $else
    }
}

/// Despawn all entities in the world.
pub fn despawn_all(mut commands: Commands, entities: Query<Entity>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

/// Despawn entity that has specified component in the world.
pub fn despawn<T: Component>(mut commands: Commands, entity: Query<Entity, With<T>>) {
    commands.entity(entity.single()).despawn();
}
