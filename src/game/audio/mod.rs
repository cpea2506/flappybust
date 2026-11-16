automod::dir!(pub "src/game/audio");

use bevy::{audio::Volume, prelude::*};
use components::AmbientMusic;
use messages::AudioMessage;

/// Audio logic.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<AudioMessage>()
            .add_systems(Update, play_audio);
    }
}

fn play_audio(mut commands: Commands, mut audio_messages: MessageReader<AudioMessage>) {
    for audio_message in audio_messages.read() {
        let player = AudioPlayer::new(audio_message.source.clone());
        let volume = Volume::Linear(audio_message.volume);

        if audio_message.looped {
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
