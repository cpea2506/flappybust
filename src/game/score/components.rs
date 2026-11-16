use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub current: usize,
    pub highest: usize,
}

#[derive(Component)]
pub(super) struct CurrentScore;
