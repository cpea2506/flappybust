use bevy::prelude::{Component, Handle, Image, Resource};

#[derive(Component)]
pub struct ScoreRank(pub Rank);

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct HighScoreText;

#[derive(Clone, Copy, PartialEq)]
pub enum Rank {
    Unit,
    Ten,
    Hunred,
}

#[derive(Resource, Clone, Default)]
pub struct Score {
    pub current: usize,
    pub highest: usize,
    pub textures: Vec<Handle<Image>>,
}

impl Score {
    pub const WIDTH: f32 = 24.;
    pub const HEIGHT: f32 = 36.;
}
