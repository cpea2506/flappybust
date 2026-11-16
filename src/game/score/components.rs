use bevy::prelude::*;

#[derive(Resource)]
pub struct Score {
    pub current: usize,
    pub highest: usize,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            current: 0,
            highest: super::load_highest(),
        }
    }
}

#[derive(Component)]
pub(super) struct CurrentScore;
