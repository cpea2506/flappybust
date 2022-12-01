use bevy::prelude::*;

#[derive(Component)]
pub struct StartMessage;

impl StartMessage {
    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("images/start_message.png"),
                transform: Transform::from_xyz(0., 73.5, 0.1),
                ..default()
            },
            StartMessage,
        ));
    }
}
