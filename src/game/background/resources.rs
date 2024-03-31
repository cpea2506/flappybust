use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct BackgroundAssets {
    #[asset(path = "images/bg_day.png")]
    pub day: Handle<Image>,
    #[asset(path = "images/bg_night.png")]
    pub night: Handle<Image>,
}
