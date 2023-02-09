#![feature(let_chains)]

mod constants;
mod game;

use bevy::{prelude::*, window::close_on_esc};
use bevy_kira_audio::prelude::*;
use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use game::{game_over::events::RestartButtonDisplayed, GamePlugin};
use iyes_loopless::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Ready,
    Playing,
    Over,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: String::from("Flappybust 🦀🦋"),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            position: WindowPosition::At(Vec2 { x: 1100., y: 256. }),
            ..default()
        },
        ..default()
    }))
    .add_plugin(AudioPlugin)
    .add_loopless_state(GameState::Ready)
    .add_enter_system(GameState::Ready, camera_setup)
    .add_plugin(GamePlugin)
    .add_system(input_setup) // event trigger on keyboard input
    .add_system(close_on_esc)
    .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn input_setup(
    mut commands: Commands,
    keyboards: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    game_state: Res<CurrentState<GameState>>,
    restart_btn_event: EventReader<RestartButtonDisplayed>,
) {
    if keyboards.just_pressed(KeyCode::Space) || buttons.just_pressed(MouseButton::Left) {
        match game_state.0 {
            GameState::Ready => commands.insert_resource(NextState(GameState::Playing)),
            GameState::Over => {
                if restart_btn_event.is_empty() {
                    return;
                }

                commands.insert_resource(NextState(GameState::Ready));
            }
            GameState::Playing => {}
        }
    }
}
