use bevy::prelude::*;

#[derive(Default, Resource)]
pub(super) enum BouncingState {
    #[default]
    Up,
    Down,
}
