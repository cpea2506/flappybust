automod::dir!("src/game");

mod audio;
mod background;
mod base;
mod bird;
mod collision;
mod game_over;
mod pipe;
mod ready_message;
mod score;

use crate::GameState;
use audio::{AudioPlugin, components::AmbientMusic, messages::AudioMessage};
use background::BackgroundPlugin;
use base::BasePlugin;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bird::BirdPlugin;
use collision::CollisionPlugin;
use flappybust::despawn;
use game_over::GameOverPlugin;
use pipe::PipePlugin;
use ready_message::ReadyMessagePlugin;
pub use resources::*;
use score::ScorePlugin;

/// Draw order (z) of every sprite and text, back to front.
mod layer {
    pub const BACKGROUND: f32 = 0.0;
    pub const PIPE: f32 = 0.1;
    pub const READY_MESSAGE: f32 = 0.1;
    /// Current score, game over text, scoreboard and restart button.
    pub const HUD: f32 = 0.2;
    pub const BIRD: f32 = 0.3;
    pub const SCOREBOARD_SCORE: f32 = 0.3;
    pub const BASE: f32 = 0.4;
    pub const MEDAL: f32 = 0.4;
    pub const BIRD_SOUL: f32 = 0.5;
}

const AMBIENT_VOLUME: f32 = 0.2;

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
            .add_systems(Update, start_game.run_if(in_state(GameState::Ready)))
            .add_systems(OnEnter(GameState::Playing), play_ambient_music)
            .add_systems(OnExit(GameState::Playing), despawn::<AmbientMusic>)
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

fn play_ambient_music(
    mut audio_message: MessageWriter<AudioMessage>,
    audio_assets: Res<AudioAssets>,
) {
    audio_message.write(AudioMessage::new_with_volume(
        &audio_assets.theme,
        true,
        AMBIENT_VOLUME,
    ));
}

fn start_game(
    key: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if key.just_pressed(KeyCode::Space)
        || mouse.just_pressed(MouseButton::Left)
        || touches.any_just_pressed()
    {
        next_state.set(GameState::Playing);
    }
}
