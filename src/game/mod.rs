mod collisions;
use collisions::CollisionPlugin;

mod audio;
use audio::AudioPlugin;

mod background;
use background::BackgroundPlugin;

mod base;
use base::BasePlugin;

mod bird;
use bird::BirdPlugin;

mod datetime;
use datetime::DateTime;

mod game_over;
use game_over::GameOverPlugin;

mod pipe;
use pipe::PipePlugin;

mod score;
use score::ScorePlugin;

mod start_message;
use start_message::StartMessagePlugin;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use rand::random;

use crate::{utils::despawn_all, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_exit_system(GameState::Over, despawn_all)
            .add_plugin(AudioPlugin)
            .insert_resource(random::<DateTime>())
            .add_plugin(StartMessagePlugin)
            .add_plugin(BackgroundPlugin)
            .add_plugin(BasePlugin)
            .add_plugin(BirdPlugin)
            .add_plugin(PipePlugin)
            .add_plugin(CollisionPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(GameOverPlugin);
    }
}
