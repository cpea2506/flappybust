use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct ScoreAssets {
    #[asset(path = "fonts/Teko-Bold.ttf")]
    pub teko_bold: Handle<Font>,
}
