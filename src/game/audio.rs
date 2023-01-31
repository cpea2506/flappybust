use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, AssetCollectionApp};
use bevy_kira_audio::prelude::*;

pub struct AudioEvent {
    pub audio: Handle<AudioSource>,
}

impl AudioEvent {
    pub fn new(audio: Handle<AudioSource>) -> Self {
        AudioEvent { audio }
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "sounds/theme.ogg")]
    pub theme: Handle<AudioSource>,
    #[asset(path = "sounds/die.ogg")]
    pub die: Handle<AudioSource>,
    #[asset(path = "sounds/ding.ogg")]
    pub ding: Handle<AudioSource>,
    #[asset(path = "sounds/heaven.ogg")]
    pub heaven: Handle<AudioSource>,
    #[asset(path = "sounds/hit.ogg")]
    pub hit: Handle<AudioSource>,
    #[asset(path = "sounds/score.ogg")]
    pub score: Handle<AudioSource>,
    #[asset(path = "sounds/swoosh.ogg")]
    pub swoosh: Handle<AudioSource>,
    #[asset(path = "sounds/wing.ogg")]
    pub wing: Handle<AudioSource>,
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AudioEvent>()
            .add_system(on_audio_event)
            .init_collection::<AudioAssets>();
    }
}

fn on_audio_event(audio: Res<Audio>, mut audio_events: EventReader<AudioEvent>) {
    if audio_events.is_empty() {
        return;
    }

    for event in audio_events.iter() {
        audio.play(event.audio.clone());
    }
}
