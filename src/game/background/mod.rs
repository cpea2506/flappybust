automod::dir!("src/game/background");

use super::{DateTime, ImageAssets};
use crate::{GAME_SPEED, GameState, SCREEN_WIDTH};
use bevy::prelude::*;
use components::Background;
use flappybust::frames;

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
    let texture = image_assets.background(*datetime);

    commands.spawn_batch(vec![
        background.generate_bundle(texture),
        secondary_background.generate_bundle(texture),
    ]);
}

fn replace_texture(
    mut backgrounds: Query<&mut Sprite, With<Background>>,
    datetime: Res<DateTime>,
    image_assets: Res<ImageAssets>,
) {
    let texture = image_assets.background(*datetime);

    for mut sprite in &mut backgrounds {
        sprite.image = texture.clone();
    }
}

fn has_existed(background: Query<(), With<Background>>) -> bool {
    !background.is_empty()
}

fn moving(time: Res<Time>, mut background: Query<(&mut Background, &mut Transform)>) {
    for (mut background, mut transform) in &mut background {
        background.translation.x =
            (background.translation.x - GAME_SPEED * frames(&time)) % SCREEN_WIDTH;

        if background.secondary {
            transform.translation.x = background.translation.x + SCREEN_WIDTH;
        } else {
            transform.translation.x = background.translation.x;
        }
    }
}
