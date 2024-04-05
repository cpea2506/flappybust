automod::dir!("src/game/background");

use components::Background;
use resources::BackgroundAssets;

use super::date_time::DateTime;
use crate::{GameState, SCREEN_WIDTH};
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollectionApp;
use flappybust::ternary;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<BackgroundAssets>()
            .add_systems(OnEnter(GameState::Ready), spawn)
            .add_systems(Update, moving.run_if(not(in_state(GameState::Over))));
    }
}

fn spawn(
    mut commands: Commands,
    background: Query<(), With<Background>>,
    background_assets: Res<BackgroundAssets>,
    datetime: Res<DateTime>,
) {
    if !background.is_empty() {
        return;
    }

    let background = Background::default();
    let secondary_background =
        Background::new(background.translation.x, background.translation.y, true);
    let texture = match *datetime {
        DateTime::Day => background_assets.day.clone(),
        DateTime::Night => background_assets.night.clone(),
    };

    commands.spawn_batch(vec![
        background.generate_bundle(&texture),
        secondary_background.generate_bundle(&texture),
    ]);
}

fn moving(mut background: Query<(&mut Background, &mut Transform)>) {
    for (mut background, mut transform) in &mut background {
        background.translation.x = (background.translation.x - 1.5f32) % SCREEN_WIDTH;

        transform.translation.x =
            background.translation.x + ternary!(background.secondary, SCREEN_WIDTH, 0f32);
    }
}
