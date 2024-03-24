use bevy::ecs::event::Event;

/// Event triggers when scoreboard is in place.
#[derive(Default, Event)]
pub struct ScoreboardDisplayed;

/// Event triggers when medal is in place.
#[derive(Default, Event)]
pub struct MedalDisplayed;

/// Event triggers when restart button is in place.
#[derive(Default, Event)]
pub struct RestartButtonDisplayed;

/// Event triggers when game over table is in place.
#[derive(Default, Event)]
pub struct GameOverTextDisplayed;
