automod::dir!("src/game/background");

use super::{DateTime, ImageAssets};
use crate::{GameState, SCREEN_WIDTH};
use bevy::prelude::*;
use components::Background;

/// Background logic.
pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Ready),
            (
                spawn.run_if(not(has_existed)),
                replace_texture.run_if(has_existed),
            ),
        )
        .add_systems(Update, moving.run_if(not(in_state(GameState::Over))));
    }
}

fn spawn(mut commands: Commands, image_assets: Res<ImageAssets>, datetime: Res<DateTime>) {
    let background = Background::new(0f32, 0f32, false);
    let secondary_background =
        Background::new(background.translation.x, background.translation.y, true);
    let texture = match *datetime {
        DateTime::Day => image_assets.bg_day.clone(),
        DateTime::Night => image_assets.bg_night.clone(),
    };

    commands.spawn_batch(vec![
        background.generate_bundle(&texture),
        secondary_background.generate_bundle(&texture),
    ]);
}

fn replace_texture(
    mut backgrounds: Query<&mut Handle<Image>, With<Background>>,
    datetime: Res<DateTime>,
    image_assets: Res<ImageAssets>,
) {
    let new_texture = match *datetime {
        DateTime::Day => image_assets.bg_day.clone(),
        DateTime::Night => image_assets.bg_night.clone(),
    };

    for mut texture in &mut backgrounds {
        if *texture == new_texture {
            return;
        }

        *texture = new_texture.clone_weak();
    }
}

fn has_existed(background: Query<(), With<Background>>) -> bool {
    !background.is_empty()
}

fn moving(mut background: Query<(&mut Background, &mut Transform)>) {
    for (mut background, mut transform) in &mut background {
        background.translation.x = (background.translation.x - 1.5f32) % SCREEN_WIDTH;

        if background.secondary {
            transform.translation.x = background.translation.x + SCREEN_WIDTH;
        } else {
            transform.translation.x = background.translation.x;
        }
    }
}
