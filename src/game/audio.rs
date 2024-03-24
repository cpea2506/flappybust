use bevy::{audio::Volume, prelude::*};
use bevy_asset_loader::prelude::{AssetCollection, AssetCollectionApp};

#[derive(Component)]
pub struct AmbientMusic;

#[derive(Event)]
pub struct AudioEvent {
    source: Handle<AudioSource>,
    looped: bool,
    volume: f32,
}

impl AudioEvent {
    pub fn new(audio: &Handle<AudioSource>, looped: bool) -> Self {
        AudioEvent {
            source: audio.clone(),
            looped,
            volume: 1.0,
        }
    }

    pub fn new_with_volume(audio: &Handle<AudioSource>, looped: bool, volume: f32) -> Self {
        AudioEvent {
            source: audio.clone(),
            looped,
            volume,
        }
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
            .init_collection::<AudioAssets>()
            .add_systems(Update, play_audio);
    }
}

fn play_audio(mut commands: Commands, mut audio_events: EventReader<AudioEvent>) {
    if audio_events.is_empty() {
        return;
    }

    for audio_event in audio_events.read() {
        if audio_event.looped {
            commands.spawn((
                AudioBundle {
                    source: audio_event.source.clone(),
                    settings: PlaybackSettings::LOOP.with_volume(Volume::new(audio_event.volume)),
                },
                AmbientMusic,
            ));
        } else {
            commands.spawn(AudioBundle {
                source: audio_event.source.clone(),
                settings: PlaybackSettings::ONCE.with_volume(Volume::new(audio_event.volume)),
            });
        }
    }
}
