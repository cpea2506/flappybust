use bevy::prelude::*;
use flappybust::Math;
use iyes_loopless::prelude::ConditionSet;

use crate::{score::Score, GameState};

#[derive(Component)]
pub struct Scoreboard;

impl Scoreboard {
    #[allow(unused)]
    const WIDTH: f32 = 226.;
    const HEIGHT: f32 = 114.;
}

fn scoreboard_spawn(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., Scoreboard::HEIGHT.half(), 0.2),
            ..default()
        },
        Scoreboard,
    ));
}

fn scoreboard_display(
    mut scoreboard: Query<&mut Handle<Image>, (With<Scoreboard>, Without<GameOver>)>,
    asset_server: Res<AssetServer>,
) {
    let mut table_texture = scoreboard.single_mut();
    *table_texture = asset_server.load("images/scoreboard.png");
}

#[derive(Component)]
pub struct GameOver;

impl GameOver {
    pub const HEIGHT: f32 = 42.;
}

fn gameover_spawn(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., Scoreboard::HEIGHT + GameOver::HEIGHT, 0.2),
            ..default()
        },
        GameOver,
    ));
}

fn gameover_display(
    mut gameover: Query<&mut Handle<Image>, With<GameOver>>,
    asset_server: Res<AssetServer>,
) {
    let mut gameover_texture = gameover.single_mut();
    *gameover_texture = asset_server.load("images/gameover.png");
}

#[derive(Component)]
pub struct Medal;

fn medal_spawn(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-65., Scoreboard::HEIGHT.half() - 10., 0.3),
            ..default()
        },
        Medal,
    ));
}

fn medal_display(
    mut medal: Query<&mut Handle<Image>, With<Medal>>,
    score: Res<Score>,
    asset_server: Res<AssetServer>,
) {
    let mut medal_texture = medal.single_mut();
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

    if let Some(name) = medal_name {
        *medal_texture = asset_server.load(format!("images/medal_{name}.png"));
    }
}

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(scoreboard_spawn)
            .add_startup_system(gameover_spawn)
            .add_startup_system(medal_spawn)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Over)
                    .with_system(gameover_display)
                    .with_system(scoreboard_display)
                    .with_system(medal_display)
                    .into(),
            );
    }
}
