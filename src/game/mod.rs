automod::dir!("src/game");

mod collision;
use collision::CollisionPlugin;

pub mod audio;
use audio::{components::AmbientMusic, events::AudioEvent, resources::AudioAssets, AudioPlugin};

mod background;
use background::BackgroundPlugin;

mod base;
use base::BasePlugin;

use date_time::DateTime;

mod bird;
use bird::BirdPlugin;

pub mod game_over;
use game_over::GameOverPlugin;

pub mod pipe;
use pipe::PipePlugin;

mod score;
use score::ScorePlugin;

mod ready_message;
use ready_message::ReadyMessagePlugin;

use bevy::prelude::*;

use crate::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DateTime>()
            .add_plugins((
                AudioPlugin,
                ReadyMessagePlugin,
                BackgroundPlugin,
                BasePlugin,
                BirdPlugin,
                PipePlugin,
                CollisionPlugin,
                ScorePlugin,
                GameOverPlugin,
            ))
            .add_systems(OnExit(GameState::Over), (init_datetime, stop_all_songs))
            .add_systems(OnEnter(GameState::Playing), play_ambient_music)
            .add_systems(OnExit(GameState::Playing), stop_ambient_music);
    }
}

fn init_datetime(mut commands: Commands) {
    commands.insert_resource(DateTime::default())
}

fn stop_all_songs(audio_sinks: Query<&AudioSink>) {
    for sink in &audio_sinks {
        sink.stop();
    }
}

fn play_ambient_music(mut audio_event: EventWriter<AudioEvent>, audio_assets: Res<AudioAssets>) {
    audio_event.send(AudioEvent::new_with_volume(&audio_assets.theme, true, 0.2));
}

fn stop_ambient_music(ambient_sinks: Query<&AudioSink, With<AmbientMusic>>) {
    for sink in &ambient_sinks {
        sink.stop();
    }
}
