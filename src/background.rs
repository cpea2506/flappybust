use crate::DateTime;
use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy)]
pub struct Background;

impl Background {
    pub const WIDTH: f32 = 288.;
    pub const HEIGHT: f32 = 512.;

    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, datetime: Res<DateTime>) {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(&format!("images/bg_{}.png", datetime.raw_value())),
                ..default()
            },
            Background,
        ));
    }
}
