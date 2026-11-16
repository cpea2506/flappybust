automod::dir!(pub "src/game/audio");

use bevy::{audio::Volume, prelude::*};
use components::AmbientMusic;
use events::AudioEvent;

/// Audio logic.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<AudioEvent>()
            .add_systems(Update, play_audio);
    }
}

fn play_audio(mut commands: Commands, mut audio_events: MessageReader<AudioEvent>) {
    if audio_events.is_empty() {
        return;
    }

    for audio_event in audio_events.read() {
        if audio_event.looped {
            commands.spawn((
                AudioPlayer::new(audio_event.source.clone()),
                PlaybackSettings::LOOP.with_volume(Volume::Linear(audio_event.volume)),
                AmbientMusic,
            ));
        } else {
            commands.spawn((
                AudioPlayer::new(audio_event.source.clone()),
                PlaybackSettings::ONCE.with_volume(Volume::Linear(audio_event.volume)),
            ));
        }
    }
}
