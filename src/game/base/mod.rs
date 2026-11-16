automod::dir!(pub "src/game/base");

use super::{ImageAssets, layer};
use crate::{GAME_SPEED, GameState, SCREEN_HEIGHT};
use bevy::prelude::*;
use components::Base;
use flappybust::{Position, Scroll};

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), spawn);
    }
}

fn spawn(mut commands: Commands, image_assets: Res<ImageAssets>) {
    let y = (Base::HEIGHT - SCREEN_HEIGHT) / 2.0;

    for x in [0f32, Base::PERIOD] {
        commands.spawn((
            Sprite::from_image(image_assets.base.clone()),
            Position(Vec3::new(x, y, layer::BASE)),
            Scroll {
                speed: GAME_SPEED,
                period: Base::PERIOD,
            },
        ));
    }
}
