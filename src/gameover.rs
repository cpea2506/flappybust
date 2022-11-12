use bevy::prelude::*;
use flappybust::Math;

use crate::score::Score;

#[derive(Component)]
pub struct Scoreboard;

impl Scoreboard {
    fn width() -> f32 {
        226.
    }

    pub fn height() -> f32 {
        114.
    }

    pub fn spawn(mut commands: Commands) {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(0., Scoreboard::height().half(), 0.2),
                ..default()
            })
            .insert(Scoreboard);
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
    fn width() -> f32 {
        192.
    }

    fn height() -> f32 {
        42.
    }

    pub fn spawn(mut commands: Commands) {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(0., Scoreboard::height() + GameOver::height(), 0.2),
                ..default()
            })
            .insert(GameOver);
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
    fn width() -> f32 {
        44.
    }

    fn height() -> f32 {
        44.
    }

    pub fn spawn(mut commands: Commands, score_board: Query<&Transform, With<Scoreboard>>) {
        let score_board = score_board.single();

        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(
                    score_board.translation.x - 65.,
                    score_board.translation.y - 10.,
                    0.3,
                ),
                ..default()
            })
            .insert(Medal);
    }

    pub fn display(
        mut medal: Query<&mut Handle<Image>, With<Medal>>,
        score: Res<Score>,
        asset_server: Res<AssetServer>,
    ) {
        let mut medal_texture = medal.single_mut();
        let mut medal_name = "bronze";

        if score.value >= 10 && score.value < 20 {
            medal_name = "silver";
        } else if score.value >= 20 && score.value < 30 {
            medal_name = "gold";
        } else if score.value >= 30 {
            medal_name = "platinum";
        }

        *medal_texture = asset_server.load(&format!("images/medal_{medal_name}.png"));
    }
}
