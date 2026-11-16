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
    for audio_event in audio_events.read() {
        let player = AudioPlayer::new(audio_event.source.clone());
        let volume = Volume::Linear(audio_event.volume);

        if audio_event.looped {
            commands.spawn((
                player,
                PlaybackSettings::LOOP.with_volume(volume),
                AmbientMusic,
            ));
        } else {
            // Despawn one-shot sounds once finished so they don't pile up.
            commands.spawn((player, PlaybackSettings::DESPAWN.with_volume(volume)));
        }
    }
}
