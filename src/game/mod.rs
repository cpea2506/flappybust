automod::dir!("src/game");

mod background;
mod base;
mod bird;
mod game_over;
mod pipe;
mod score;

use crate::GameState;
use background::BackgroundPlugin;
use base::BasePlugin;
use bevy::{audio::Volume, prelude::*};
use bevy_asset_loader::prelude::*;
use bird::BirdPlugin;
use flappybust::{flap_pressed, scroll};
use game_over::GameOverPlugin;
use pipe::PipePlugin;
pub use resources::*;
use score::ScorePlugin;

/// Draw order (z) of every sprite and text, back to front.
mod layer {
    pub const BACKGROUND: f32 = 0.0;
    pub const PIPE: f32 = 0.1;
    pub const COIN: f32 = 0.15;
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
const READY_MESSAGE_Y: f32 = 73.5;

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
                BackgroundPlugin,
                BasePlugin,
                BirdPlugin,
                PipePlugin,
                ScorePlugin,
                GameOverPlugin,
            ))
            .add_systems(OnEnter(GameState::Ready), spawn_ready_message)
            .add_systems(
                Update,
                start_game
                    .run_if(in_state(GameState::Ready))
                    .run_if(flap_pressed),
            )
            .add_systems(OnEnter(GameState::Playing), play_ambient_music)
            .add_systems(Update, scroll.run_if(not(in_state(GameState::Over))))
            .add_systems(OnExit(GameState::Over), init_datetime);
    }
}

fn init_datetime(mut commands: Commands) {
    commands.insert_resource(DateTime::default())
}

fn spawn_ready_message(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        Sprite::from_image(image_assets.ready_message.clone()),
        Transform::from_xyz(0f32, READY_MESSAGE_Y, layer::READY_MESSAGE),
        DespawnOnExit(GameState::Ready),
    ));
}

fn play_ambient_music(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn((
        AudioPlayer::new(audio_assets.theme.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(AMBIENT_VOLUME)),
        DespawnOnExit(GameState::Playing),
    ));
}

fn start_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}
