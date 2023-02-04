use bevy::prelude::Component;
use strum::AsRefStr;

#[derive(AsRefStr, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum MedalType {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

#[derive(Component)]
pub struct Medal(pub Option<MedalType>);

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct Scoreboard {
    pub velocity: f32,
    pub gravity: f32,
}

#[derive(Component)]
pub struct GameOverText;
