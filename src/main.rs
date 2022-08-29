mod background;
mod base;
mod bird;
mod gameover;
mod pipe;
mod score;
mod start_message;

use background::Background;
use base::Base;
use bevy::{input::system::exit_on_esc_system, prelude::*};
use bird::Bird;
use iyes_loopless::prelude::*;
use pipe::Pipe;
use start_message::StartMessage;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Ready,
    Pausing,
    Playing,
    Over,
}

pub const FPS: f32 = 1. / 60.;

fn main() {
    let mut app = App::new();
    let default_window = WindowDescriptor {
        title: String::from("Flappy Crab"),
        width: 288.,
        height: 512.,
        position: Some(Vec2::new(1000., 250.)),
        ..default()
    };

    app.insert_resource(default_window)
        .add_plugins(DefaultPlugins)
        .add_plugin(StartupPlugin)
        .add_loopless_state(GameState::Ready) // add initial game state
        .add_plugin(PlayingPlugin) // in playing state
        .add_system(keyboard_input_system) // event trigger on keyboard input
        .add_system(exit_on_esc_system)
        .run();
}

struct StartupPlugin;
struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(Background::moving_system)
                .with_system(Base::moving_system)
                .with_system(Bird::flap_system)
                .with_system(Pipe::moving_system)
                .into(),
        );
    }
}

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_system)
            .add_startup_system(Background::startup_system)
            .add_startup_system(Base::startup_system)
            .add_startup_system(StartMessage::startup_system)
            .add_startup_system(Bird::startup_system)
            .add_startup_system(Pipe::startup_system);
    }
}

fn startup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn keyboard_input_system(
    mut commands: Commands,
    game_state: Res<CurrentState<GameState>>,
    start_message: Res<StartMessage>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        match game_state.0 {
            GameState::Ready => {
                commands.entity(start_message.entity).despawn();

                // change game state to playing
                commands.insert_resource(NextState(GameState::Playing));
            }
            GameState::Over => {
                // change game state to playing
                commands.insert_resource(NextState(GameState::Ready));

                StartMessage::startup_system(commands, asset_server);
            }
            GameState::Playing => {
                // game state is still playing
                commands.insert_resource(NextState(GameState::Playing));
            }
            GameState::Pausing => todo!(),
        }
    }
}
