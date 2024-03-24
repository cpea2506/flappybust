use crate::GameState;
use bevy::prelude::*;
use flappybust::despawn;

#[derive(Component)]
struct StartMessage;

pub struct StartMessagePlugin;

impl Plugin for StartMessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Ready), spawn)
            .add_systems(OnEnter(GameState::Playing), despawn::<StartMessage>);
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/start_message.png"),
            transform: Transform::from_xyz(0f32, 73.5, 0.1),
            ..default()
        },
        StartMessage,
    ));
}
