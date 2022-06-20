mod background;
mod base;
mod bird;
mod gameover;
mod pipe;
mod score;

use background::{Background, Nychthemeron};
use base::Base;
use bevy::{input::system::exit_on_esc_system, prelude::*};

enum GameState {
    Ready,
    Playing,
    Over,
}

fn main() {
    let mut app = App::new();
    let default_window = WindowDescriptor {
        title: "Flappy Crab".to_string(),
        width: 288.0,
        height: 512.0,
        ..default()
    };

    app.insert_resource(default_window)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(exit_on_esc_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: Res<Windows>) {
    let nychthemeron = rand::random::<Nychthemeron>();
    let background_image: Handle<Image> = asset_server.load(&format!(
        "images/background-{}.png",
        nychthemeron.raw_value()
    ));
    let bird_image = asset_server.load("images/redbird-midflap.png");
    let base_image = asset_server.load("images/base.png");
    let message_image = asset_server.load("images/message.png");
    let window = window.get_primary().unwrap();

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // background
    commands
        .spawn_bundle(SpriteBundle {
            texture: background_image,
            ..default()
        })
        .insert(Background::default());

    // base
    commands
        .spawn_bundle(SpriteBundle {
            texture: base_image,
            transform: Transform::from_xyz(0.0, 56.0 - window.height() / 2.0, 0.1),
            ..default()
        })
        .insert(Base::default());

    // message
    commands.spawn_bundle(SpriteBundle {
        texture: message_image,
        transform: Transform::from_xyz(0.0, 73.5, 0.1),
        ..default()
    });

    // bird
    commands.spawn_bundle(SpriteBundle {
        texture: bird_image,
        transform: Transform::from_xyz(-53.0, 12.0, 0.1),
        ..default()
    });
}
