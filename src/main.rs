mod audio;
mod background;
mod base;
mod bird;
mod datetime;
mod gameover;
mod pipe;
mod score;
mod start_message;

use audio::FlappyAudioPlugin;
use background::{Background, BackgroundPlugin};
use base::BasePlugin;
use bevy::{prelude::*, window::close_on_esc};
use bevy_kira_audio::prelude::*;
use bird::BirdPlugin;
use datetime::DateTime;
use gameover::GameOverPlugin;
use iyes_loopless::prelude::*;
use pipe::PipePlugin;
use rand::random;
use score::ScorePlugin;
use start_message::{StartMessage, StartMessagePlugin};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Ready,
    Playing,
    Over,
}

const FPS: f32 = 1. / 100.;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: String::from("Flappybust"),
            width: Background::WIDTH,
            height: Background::HEIGHT,
            position: WindowPosition::At(Vec2 { x: 1100., y: 256. }),
            ..default()
        },
        ..default()
    }))
    .add_startup_system(default_setup)
    .add_loopless_state(GameState::Ready)
    .insert_resource(random::<DateTime>())
    .add_plugin(AudioPlugin)
    .add_plugin(StartMessagePlugin)
    .add_plugin(FlappyAudioPlugin)
    .add_plugin(BackgroundPlugin)
    .add_plugin(BasePlugin)
    .add_plugin(BirdPlugin)
    .add_plugin(PipePlugin)
    .add_plugin(ScorePlugin)
    .add_plugin(GameOverPlugin)
    .add_system(input_system) // event trigger on keyboard input
    .add_system(close_on_esc)
    .run();
}

fn default_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn cleanup(mut commands: Commands, entities: Query<Entity>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

fn input_system(
    mut commands: Commands,
    keyboards: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    game_state: Res<CurrentState<GameState>>,
    start_message: Query<Entity, With<StartMessage>>,
) {
    if (keyboards.just_pressed(KeyCode::Space) || buttons.just_pressed(MouseButton::Left))
        && GameState::Ready == game_state.0
    {
        commands.entity(start_message.single()).despawn();

        // change game state to playing
        commands.insert_resource(NextState(GameState::Playing));
    }
}
