use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use flappybust::despawn;

use crate::GameState;

#[derive(Component)]
struct StartMessage;

pub struct StartMessagePlugin;

impl Plugin for StartMessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Ready, spawn)
            .add_enter_system(GameState::Playing, despawn::<StartMessage>);
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/start_message.png"),
            transform: Transform::from_xyz(0., 73.5, 0.1),
            ..default()
        },
        StartMessage,
    ));
}
