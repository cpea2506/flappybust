automod::dir!(pub "src/game/base");

use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollectionApp;
use components::Base;
use flappybust::{despawn, ternary, BasicMath};

use self::resources::BaseAssets;

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<BaseAssets>()
            .add_systems(OnEnter(GameState::Ready), (despawn::<Base>, spawn))
            .add_systems(Update, moving.run_if(not(in_state(GameState::Over))));
    }
}

fn spawn(mut commands: Commands, window: Query<&Window>, base_assets: Res<BaseAssets>) {
    if let Ok(window) = window.get_single() {
        let base = Base::new(
            0f32,
            Base::HEIGHT.half() - window.resolution.height().half(),
            false,
        );
        let secondary_base = Base::new(Base::WIDTH, base.translation.y, true);
        let texture = base_assets.base.clone();

        commands.spawn_batch(vec![
            base.generate_bundle(&texture),
            secondary_base.generate_bundle(&texture),
        ]);
    }
}

fn moving(mut base: Query<(&mut Base, &mut Transform)>) {
    for (mut base, mut transform) in &mut base {
        base.translation.x = (base.translation.x - 1.5f32) % Base::RESET_POINT;

        transform.translation.x =
            base.translation.x + ternary!(base.secondary, Base::RESET_POINT, 0f32);
    }
}
