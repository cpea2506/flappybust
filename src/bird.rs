use bevy::prelude::*;

#[derive(Default)]
struct Speed {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Bird {
    speed: Speed,
}
