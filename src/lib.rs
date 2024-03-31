#![feature(decl_macro)]

use bevy::prelude::*;

/// Add useful helper methods for Boolean type.
pub trait Switcher {
    /// Change value to open state.
    fn on(&mut self);
    /// Change value value to close state.
    fn off(&mut self);
    /// Toggle a value from `open` to `close` state or from `close` to `open` state.
    fn toggle(&mut self);
}

impl Switcher for bool {
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

impl Switcher for Visibility {
    fn off(&mut self) {
        *self = Visibility::Hidden;
    }

    fn on(&mut self) {
        *self = Visibility::Visible;
    }

    fn toggle(&mut self) {
        *self = match self {
            Visibility::Hidden => Visibility::Visible,
            Visibility::Visible => Visibility::Hidden,
            _ => Visibility::Inherited,
        };
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

/// Despawn entity that has specified components in the world.
pub fn despawn<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
