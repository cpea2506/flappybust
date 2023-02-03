pub mod components;
use components::*;

use bevy::prelude::*;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use iyes_loopless::prelude::*;

use crate::GameState;

use super::score::Score;

#[derive(Component)]
struct GameOver;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            GameState::Over,
            SystemSet::new()
                .with_system(gameover_spawn)
                .with_system(scoreboard_spawn)
                .with_system(medal_spawn)
                .with_system(restart_button_spawn),
        );
    }
}

fn gameover_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 156., 0.2),
            texture: asset_server.load("images/game_over.png"),
            ..default()
        },
        GameOver,
    ));
}

pub fn restart_button_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/restart_btn.png"),
        transform: Transform::from_xyz(0., -35., 0.2),
        ..default()
    });
}

pub fn medal_spawn(mut commands: Commands, score: Res<Score>, asset_server: Res<AssetServer>) {
    let mut medal_name = None;

    if score.current >= 10 && score.current < 20 {
        medal_name = Some("bronze");
    } else if score.current >= 20 && score.current < 30 {
        medal_name = Some("silver");
    } else if score.current >= 30 && score.current < 40 {
        medal_name = Some("gold");
    } else if score.current >= 40 {
        medal_name = Some("platinum");
    }

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-65., 47., 0.2),
            texture: match medal_name {
                Some(name) => asset_server.load(format!("images/medal_{name}.png")),
                None => DEFAULT_IMAGE_HANDLE.typed(),
            },
            ..default()
        },
        Medal,
    ));
}

pub fn scoreboard_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 57., 0.2),
            texture: asset_server.load("images/scoreboard.png"),
            ..default()
        },
        Scoreboard,
    ));
}
