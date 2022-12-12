use bevy::prelude::*;
use flappybust::Math;

use crate::score::Score;

#[derive(Component)]
pub struct Scoreboard;

impl Scoreboard {
    pub const WIDTH: f32 = 226.;
    pub const HEIGHT: f32 = 114.;

    pub fn spawn(mut commands: Commands) {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0., Scoreboard::HEIGHT.half(), 0.2),
                ..default()
            },
            Scoreboard,
        ));
    }

    pub fn display(
        mut scoreboard: Query<&mut Handle<Image>, (With<Scoreboard>, Without<GameOver>)>,
        asset_server: Res<AssetServer>,
    ) {
        let mut table_texture = scoreboard.single_mut();
        *table_texture = asset_server.load("images/scoreboard.png");
    }
}

#[derive(Component)]
pub struct GameOver;

impl GameOver {
    pub const HEIGHT: f32 = 42.;

    pub fn spawn(mut commands: Commands) {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0., Scoreboard::HEIGHT + GameOver::HEIGHT, 0.2),
                ..default()
            },
            GameOver,
        ));
    }

    pub fn display(
        mut gameover: Query<&mut Handle<Image>, With<GameOver>>,
        asset_server: Res<AssetServer>,
    ) {
        let mut gameover_texture = gameover.single_mut();
        *gameover_texture = asset_server.load("images/gameover.png");
    }
}

#[derive(Component)]
pub struct Medal;

impl Medal {
    pub fn spawn(mut commands: Commands, score_board: Query<&Transform, With<Scoreboard>>) {
        let score_board = score_board.single();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    score_board.translation.x - 65.,
                    score_board.translation.y - 10.,
                    0.3,
                ),
                ..default()
            },
            Medal,
        ));
    }

    pub fn display(
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
}
