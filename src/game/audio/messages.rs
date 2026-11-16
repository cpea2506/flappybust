use bevy::prelude::*;

#[derive(Message)]
pub struct AudioMessage {
    pub(super) source: Handle<AudioSource>,
    pub(super) looped: bool,
    pub(super) volume: f32,
}

impl AudioMessage {
    pub fn new(audio: &Handle<AudioSource>, looped: bool) -> Self {
        AudioMessage {
            source: audio.clone(),
            looped,
            volume: 1.0,
        }
    }

    pub fn new_with_volume(audio: &Handle<AudioSource>, looped: bool, volume: f32) -> Self {
        AudioMessage {
            source: audio.clone(),
            looped,
            volume,
        }
    }
}
