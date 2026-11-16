automod::dir!("src/game/background");

use super::{DateTime, ImageAssets, layer};
use crate::GameState;
use bevy::prelude::*;
use components::Background;
use flappybust::Scroll;

/// Background logic.
pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), spawn)
            .add_systems(OnEnter(GameState::Ready), replace_texture);
    }
}

fn spawn(mut commands: Commands) {
    for x in [0f32, Background::PERIOD] {
        commands.spawn((
            Sprite::default(),
            Transform::from_xyz(x, 0f32, layer::BACKGROUND),
            Scroll {
                speed: Background::SPEED,
                period: Background::PERIOD,
            },
            Background,
        ));
    }
}

/// Show the day or night background picked for this round.
fn replace_texture(
    mut backgrounds: Query<&mut Sprite, With<Background>>,
    datetime: Res<DateTime>,
    image_assets: Res<ImageAssets>,
) {
    for mut sprite in &mut backgrounds {
        sprite.image = image_assets.background(*datetime).clone();
    }
}
