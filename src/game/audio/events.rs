use bevy::prelude::*;

#[derive(Event)]
pub struct AudioEvent {
    pub(super) source: Handle<AudioSource>,
    pub(super) looped: bool,
    pub(super) volume: f32,
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
