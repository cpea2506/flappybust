use bevy::prelude::*;

pub struct StartMessage {
    pub entity: Entity,
}

impl StartMessage {
    pub fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
        let texture = asset_server.load("images/start_message.png");
        let start_message = commands
            .spawn_bundle(SpriteBundle {
                texture,
                transform: Transform::from_xyz(0., 73.5, 0.1),
                ..default()
            })
            .id();

        commands.insert_resource(StartMessage {
            entity: start_message,
        });
    }
}
