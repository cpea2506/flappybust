automod::dir!("src/game/ready_message");

use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollectionApp;
use components::ReadyMessage;
use flappybust::despawn;
use resources::ReadyMessageAssets;

pub struct ReadyMessagePlugin;

impl Plugin for ReadyMessagePlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ReadyMessageAssets>()
            .add_systems(OnEnter(GameState::Ready), spawn)
            .add_systems(OnEnter(GameState::Playing), despawn::<ReadyMessage>);
    }
}

fn spawn(mut commands: Commands, ready_message_assets: Res<ReadyMessageAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: ready_message_assets.message.clone(),
            transform: Transform::from_xyz(0f32, 73.5, 0.1),
            ..default()
        },
        ReadyMessage,
    ));
}
