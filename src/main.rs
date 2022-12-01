mod background;
mod base;
mod bird;
mod datetime;
mod gameover;
mod pipe;
mod score;
mod start_message;

use std::time::Duration;

use background::Background;
use base::Base;
use bevy::{prelude::*, window::close_on_esc};
use bevy_kira_audio::prelude::*;
use bird::Bird;
use datetime::DateTime;
use gameover::{GameOver, Medal, Scoreboard};
use iyes_loopless::prelude::*;
use pipe::Pipe;
use score::Score;
use start_message::StartMessage;

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
            ..default()
        },
        ..default()
    }))
    .add_plugin(AudioPlugin)
    .add_loopless_state(GameState::Ready)
    .add_plugin(StartupPlugin)
    .add_plugin(PlayingPlugin) // in playing state
    .add_system(input_system) // event trigger on keyboard input
    .add_system(close_on_esc)
    .run();
}

struct StartupPlugin;
struct PlayingPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(default_setup)
            .add_startup_system_to_stage(StartupStage::PreStartup, DateTime::gen)
            .add_startup_system_to_stage(StartupStage::PreStartup, Scoreboard::spawn)
            .add_startup_system(Base::spawn)
            .add_startup_system(Background::spawn)
            .add_startup_system(StartMessage::spawn)
            .add_startup_system(Bird::spawn)
            .add_startup_system(Pipe::spawn)
            .add_startup_system(Score::spawn)
            .add_startup_system(GameOver::spawn)
            .add_startup_system(Medal::spawn);
    }
}

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        let mut fixed = SystemStage::parallel();

        fixed
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(Pipe::moving)
                    .with_system(Score::record)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_not_in_state(GameState::Over)
                    .with_system(Bird::flap)
                    .with_system(Base::moving)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Over)
                    .with_system(GameOver::display)
                    .with_system(Scoreboard::display)
                    .with_system(Medal::display)
                    .with_system(Score::record)
                    .into(),
            )
            .add_system(Bird::fly.run_not_in_state(GameState::Ready));

        app.add_stage_before(
            CoreStage::Update,
            "FixedUpdate",
            FixedTimestepStage::from_stage(Duration::from_secs_f32(FPS), "FixedUpdated", fixed),
        );
    }
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
    if keyboards.just_pressed(KeyCode::Space) || buttons.just_pressed(MouseButton::Left) {
        if let GameState::Ready = game_state.0 {
            commands.entity(start_message.single()).despawn();

            // change game state to playing
            commands.insert_resource(NextState(GameState::Playing));
        }
    }
}
