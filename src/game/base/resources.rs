use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub(super) struct BaseAssets {
    #[asset(path = "images/base.png")]
    pub base: Handle<Image>,
}
