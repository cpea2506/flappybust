use bevy::{ecs::component::Component, prelude::Resource};

#[derive(Resource, Copy, Clone, Default)]
pub struct Score {
    pub current: usize,
    pub highest: usize,
}

#[derive(Component)]
pub(super) struct CurrentScore;
