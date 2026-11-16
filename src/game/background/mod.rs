automod::dir!("src/game/background");

use super::{DateTime, ImageAssets, layer};
use crate::{GameState, SCREEN_WIDTH};
use bevy::prelude::*;
use components::Background;
use flappybust::{Position, Scroll};

/// Background logic.
pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), spawn)
            .add_systems(OnEnter(GameState::Ready), replace_texture);
    }
}

fn spawn(mut commands: Commands) {
    for x in [0f32, SCREEN_WIDTH] {
        commands.spawn((
            Sprite::default(),
            Position(Vec3::new(x, 0f32, layer::BACKGROUND)),
            Scroll {
                speed: Background::SPEED,
                period: SCREEN_WIDTH,
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
