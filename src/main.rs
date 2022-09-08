mod background;
mod base;
mod bird;
mod daytime;
mod gameover;
mod pipe;
mod score;
mod start_message;

use background::Background;
use base::Base;
use bevy::{prelude::*, window::close_on_esc};
use bird::Bird;
use daytime::DateTime;
use iyes_loopless::prelude::*;
use pipe::Pipe;
use start_message::StartMessage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Ready,
    Pausing,
    Playing,
    Over,
}

fn main() {
    let mut app = App::new();
    let default_window = WindowDescriptor {
        title: String::from("Flappy Bust"),
        width: 288.,
        height: 512.,
        position: WindowPosition::At(Vec2::new(1050., 365.)),
        ..default()
    };

    app.insert_resource(default_window)
        .add_plugins(DefaultPlugins)
        .add_loopless_state(GameState::Ready)
        .add_plugin(StartupPlugin)
        .add_plugin(PlayingPlugin) // in playing state
        .add_system(input_system) // event trigger on keyboard input
        .add_system(close_on_esc)
        .run();
}

struct StartupPlugin;
struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(Base::moving)
                .with_system(Background::moving)
                .with_system(Pipe::moving)
                .with_system(Bird::fly)
                .with_system(Bird::flap)
                .into(),
        )
        .add_system(Bird::fly.run_in_state(GameState::Over));
    }
}

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_startup_system_to_stage(StartupStage::PreStartup, DateTime::spawn)
            .add_startup_system(Base::spawn)
            .add_startup_system(Background::spawn)
            .add_startup_system(StartMessage::spawn)
            .add_startup_system(Bird::spawn)
            .add_startup_system(Pipe::spawn);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn input_system(
    mut commands: Commands,
    keyboards: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    game_state: Res<CurrentState<GameState>>,
    start_message: Query<Entity, With<StartMessage>>,
) {
    if keyboards.just_pressed(KeyCode::Space) || buttons.just_pressed(MouseButton::Left) {
        match game_state.0 {
            GameState::Ready => {
                commands.entity(start_message.single()).despawn();

                // change game state to playing
                commands.insert_resource(NextState(GameState::Playing));
            }
            GameState::Over => {}
            GameState::Playing => {}
            GameState::Pausing => todo!(),
        }
    }
}