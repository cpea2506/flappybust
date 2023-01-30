use super::{DateTime, GameState};
use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

#[derive(Component, Default, Clone, Copy)]
struct Background;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Ready, spawn);
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, datetime: Res<DateTime>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(format!("images/bg_{}.png", *datetime)),
            ..default()
        },
        Background,
    ));
}
