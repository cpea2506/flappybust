use bevy::prelude::Message;

/// Event triggers when scoreboard is in place.
#[derive(Default, Message)]
pub struct ScoreboardDisplayed;

/// Event triggers when medal is in place.
#[derive(Default, Message)]
pub struct MedalDisplayed;

/// Event triggers when restart button is in place.
#[derive(Default, Message)]
pub struct RestartButtonDisplayed;

/// Event triggers when game over table is in place.
#[derive(Default, Message)]
pub struct GameOverTextDisplayed;
