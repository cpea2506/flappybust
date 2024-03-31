use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub(super) struct PipeAssets {
    #[asset(path = "images/pipe_green.png")]
    pub green: Handle<Image>,
    #[asset(path = "images/pipe_red.png")]
    pub red: Handle<Image>,
}
