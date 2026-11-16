automod::dir!("src/game/ready_message");

use super::{ImageAssets, layer};
use crate::GameState;
use bevy::prelude::*;
use components::ReadyMessage;

/// Ready message logic.
pub struct ReadyMessagePlugin;

impl Plugin for ReadyMessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Ready), spawn);
    }
}

fn spawn(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        Sprite::from_image(image_assets.ready_message.clone()),
        Transform::from_xyz(0f32, ReadyMessage::Y, layer::READY_MESSAGE),
        ReadyMessage,
        DespawnOnExit(GameState::Ready),
    ));
}
