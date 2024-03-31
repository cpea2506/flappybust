use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(Default, Resource)]
pub(super) enum BouncingState {
    #[default]
    Up,
    Down,
}

#[derive(AssetCollection, Resource)]
pub(super) struct BirdAssets {
    #[asset(path = "images/bird_soul.png")]
    pub bird_soul: Handle<Image>,
}
