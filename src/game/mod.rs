automod::dir!("src/game");

pub mod audio;
mod background;
mod base;
mod bird;
mod collision;
pub mod game_over;
pub mod pipe;
mod ready_message;
mod score;

use crate::GameState;
use audio::{components::AmbientMusic, events::AudioEvent, AudioPlugin};
use background::BackgroundPlugin;
use base::BasePlugin;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bird::BirdPlugin;
use collision::CollisionPlugin;
use game_over::GameOverPlugin;
use pipe::PipePlugin;
use ready_message::ReadyMessagePlugin;
pub use resources::*;
use score::ScorePlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DateTime>()
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::Ready)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>("audios.assets.ron")
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>("images.assets.ron")
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>("fonts.assets.ron")
                    .load_collection::<ImageAssets>()
                    .load_collection::<FontAssets>()
                    .load_collection::<AudioAssets>(),
            )
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
            .add_systems(OnEnter(GameState::Playing), play_ambient_music)
            .add_systems(OnExit(GameState::Playing), stop_ambient_music)
            .add_systems(OnExit(GameState::Over), (init_datetime, stop_all_songs));
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
