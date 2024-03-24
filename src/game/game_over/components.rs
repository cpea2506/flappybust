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
pub struct Medal(Option<MedalType>);

impl Medal {
    pub fn new(medal: Option<MedalType>) -> Self {
        Self(medal)
    }

    pub fn get(&self) -> &Option<MedalType> {
        &self.0
    }
}

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct Scoreboard {
    pub velocity: f32,
    pub gravity: f32,
}

impl Default for Scoreboard {
    fn default() -> Self {
        Scoreboard {
            gravity: 0.15,
            velocity: 0f32,
        }
    }
}

#[derive(Component)]
pub struct GameOverText {
    pub velocity: f32,
    pub gravity: f32,

    /// Whether text should be bounced or not (default: true).
    pub bounce: bool,
}

impl Default for GameOverText {
    fn default() -> Self {
        GameOverText {
            gravity: 0.1,
            velocity: 0f32,
            bounce: true,
        }
    }
}
