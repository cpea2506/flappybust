use bevy::prelude::Component;

#[derive(Component, Default)]
struct Score {
    value: usize,
}
