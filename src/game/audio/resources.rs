use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;

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
