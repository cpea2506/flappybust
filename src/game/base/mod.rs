automod::dir!(pub "src/game/base");

use super::ImageAssets;
use crate::{GameState, SCREEN_HEIGHT};
use bevy::prelude::*;
use components::Base;
use flappybust::BasicMath;

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Ready), spawn)
            .add_systems(Update, moving.run_if(not(in_state(GameState::Over))));
    }
}

fn spawn(mut commands: Commands, base: Query<(), With<Base>>, image_assets: Res<ImageAssets>) {
    if !base.is_empty() {
        return;
    }

    let base = Base::new(0f32, Base::HEIGHT.half() - SCREEN_HEIGHT.half(), false);
    let secondary_base = Base::new(Base::WIDTH, base.translation.y, true);
    let texture = image_assets.base.clone();

    commands.spawn_batch(vec![
        base.generate_bundle(&texture),
        secondary_base.generate_bundle(&texture),
    ]);
}

fn moving(mut base: Query<(&mut Base, &mut Transform)>) {
    for (mut base, mut transform) in &mut base {
        base.translation.x = (base.translation.x - 1.5f32) % Base::RESET_POINT;

        if base.secondary {
            transform.translation.x = base.translation.x + Base::RESET_POINT;
        } else {
            transform.translation.x = base.translation.x;
        }
    }
}
