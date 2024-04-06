automod::dir!("src/game/ready_message");

use super::ImageAssets;
use crate::GameState;
use bevy::prelude::*;
use components::ReadyMessage;
use flappybust::despawn;

/// Ready message logic.
pub struct ReadyMessagePlugin;

impl Plugin for ReadyMessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Ready), spawn)
            .add_systems(OnEnter(GameState::Playing), despawn::<ReadyMessage>);
    }
}

fn spawn(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: image_assets.ready_message.clone(),
            transform: Transform::from_xyz(0f32, 73.5, 0.1),
            ..default()
        },
        ReadyMessage,
    ));
}
