use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct AudioEvent {
    pub audio: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct FlappyAudioAssets {
    pub theme: Handle<AudioSource>,
    pub die: Handle<AudioSource>,
    pub ding: Handle<AudioSource>,
    pub heaven: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub score: Handle<AudioSource>,
    pub swoosh: Handle<AudioSource>,
    pub wing: Handle<AudioSource>,
}

pub struct FlappyAudioPlugin;

impl Plugin for FlappyAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup)
            .add_event::<AudioEvent>()
            .add_system(on_audio_event);
    }
}

pub fn startup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let audio_assets = FlappyAudioAssets {
        theme: asset_server.load("sounds/theme.ogg"),
        die: asset_server.load("sounds/die.ogg"),
        ding: asset_server.load("sounds/ding.ogg"),
        heaven: asset_server.load("sounds/heaven.ogg"),
        hit: asset_server.load("sounds/hit.ogg"),
        score: asset_server.load("sounds/score.ogg"),
        swoosh: asset_server.load("sounds/swoosh.ogg"),
        wing: asset_server.load("sounds/wing.ogg"),
    };

    commands.insert_resource(audio_assets);
}

fn on_audio_event(audio: Res<Audio>, mut audio_events: EventReader<AudioEvent>) {
    if audio_events.is_empty() {
        return;
    }

    for event in audio_events.iter() {
        audio.play(event.audio.clone());
    }
}
