use bevy::prelude::Message;

/// Sent when scoreboard is in place.
#[derive(Default, Message)]
pub struct ScoreboardDisplayed;

/// Sent when medal is in place.
#[derive(Default, Message)]
pub struct MedalDisplayed;

/// Sent when restart button is in place.
#[derive(Default, Message)]
pub struct RestartButtonDisplayed;

/// Sent when game over table is in place.
#[derive(Default, Message)]
pub struct GameOverTextDisplayed;
