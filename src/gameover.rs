use bevy::prelude::*;
use flappybust::Math;

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
