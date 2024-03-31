use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub(super) struct ReadyMessageAssets {
    #[asset(path = "images/ready_message.png")]
    pub message: Handle<Image>,
}
