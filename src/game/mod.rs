mod collisions;
use collisions::CollisionPlugin;

mod audio;
use audio::{AudioAssets, AudioEvent, AudioPlugin, ThemeSongHandle};

mod background;
use background::BackgroundPlugin;

mod base;
use base::BasePlugin;

mod bird;
use bird::{events::DeathEvent, BirdPlugin};

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

use flappybust::despawn_all;

use crate::GameState;
use bevy_kira_audio::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_exit_system(GameState::Over, despawn_all)
            .add_plugin(AudioPlugin)
            .init_resource::<DateTime>()
            .add_exit_system(GameState::Over, init_datetime)
            .add_enter_system(GameState::Playing, play_theme_song)
            .add_exit_system(GameState::Playing, stop_theme_song)
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

fn init_datetime(mut commands: Commands) {
    commands.insert_resource(DateTime::default())
}

fn play_theme_song(mut audio_event: EventWriter<AudioEvent>, audio_assets: Res<AudioAssets>) {
    audio_event.send(AudioEvent::new(&audio_assets.theme, true));
}

fn stop_theme_song(
    handle: Res<ThemeSongHandle>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&handle.0) {
        if let PlaybackState::Playing { .. } = instance.state() {
            instance.stop(AudioTween::default());
        }
    }
}
