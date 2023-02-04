pub mod components;
use components::*;

mod events;
use events::*;

use bevy::prelude::*;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use iyes_loopless::prelude::*;

use crate::GameState;

use super::audio::{AudioAssets, AudioEvent};
use super::bird::events::DeathEvent;
use super::score::components::{HighScoreText, Score, ScoreText};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScoreboardEvent>()
            .add_enter_system_set(
                GameState::Over,
                SystemSet::new()
                    .with_system(gameover_spawn)
                    .with_system(medal_spawn),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Over)
                    .with_system(medal_scale)
                    .with_system(scoreboard_moving_up)
                    .into(),
            );
    }
}

fn gameover_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 156., 0.2),
            texture: asset_server.load("images/game_over.png"),
            ..default()
        },
        GameOverText,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., -199., 0.2),
            texture: asset_server.load("images/scoreboard.png"),
            ..default()
        },
        Scoreboard {
            velocity: 0.,
            gravity: 0.15,
        },
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/restart_btn.png"),
            transform: Transform::from_xyz(0., -35., 0.2),
            ..default()
        },
        RestartButton,
    ));
}

fn scoreboard_moving_up(
    mut scoreboard: Query<(&mut Transform, &mut Scoreboard)>,
    mut score_text: Query<&mut Visibility, With<ScoreText>>,
    mut high_score_text: Query<&mut Visibility, (With<HighScoreText>, Without<ScoreText>)>,
    mut scoreboard_event: EventWriter<ScoreboardEvent>,
) {
    let (mut transform, mut scoreboard) = scoreboard.single_mut();

    scoreboard.velocity += scoreboard.gravity;

    transform.translation.y = (transform.translation.y + scoreboard.velocity).clamp(-199., 57.);

    if transform.translation.y == 57. {
        let mut score_text = score_text.single_mut();
        let mut high_score_text = high_score_text.single_mut();

        score_text.toggle();
        high_score_text.toggle();

        scoreboard_event.send_default();
    }
}

fn medal_spawn(mut commands: Commands, score: Res<Score>, asset_server: Res<AssetServer>) {
    let mut medal_name = None;

    if score.current >= 10 && score.current < 20 {
        medal_name = Some(MedalType::Bronze);
    } else if score.current >= 20 && score.current < 30 {
        medal_name = Some(MedalType::Silver);
    } else if score.current >= 30 && score.current < 40 {
        medal_name = Some(MedalType::Gold);
    } else if score.current >= 40 {
        medal_name = Some(MedalType::Platinum);
    }

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                // -65., 47.
                translation: Vec3::new(-65., 47., 0.4),
                scale: Vec3::new(25., 25., 0.),
                ..Transform::IDENTITY
            },
            visibility: Visibility::INVISIBLE,
            texture: match medal_name {
                Some(name) => asset_server.load(format!("images/medal_{}.png", name.as_ref())),
                None => DEFAULT_IMAGE_HANDLE.typed(),
            },
            ..default()
        },
        Medal(medal_name),
    ));
}

fn medal_scale(
    mut medal: Query<(&mut Transform, &mut Visibility, &Medal)>,
    mut audio_event: EventWriter<AudioEvent>,
    audio_assets: Res<AudioAssets>,
    death_event: EventReader<DeathEvent>,
    scoreboard_event: EventReader<ScoreboardEvent>,
) {
    if scoreboard_event.is_empty() || death_event.is_empty() {
        return;
    }

    let (mut transform, mut visibility, medal) = medal.single_mut();

    if medal.0.is_none() {
        return;
    }

    visibility.is_visible = true;

    // scale both xy for circle
    let scale_direction = Vec3::X + Vec3::Y;

    // scale to orignal state
    transform.scale = (transform.scale - scale_direction).clamp_length_min(Vec3::ONE.length());

    // play audio if and only if the scale length reachs
    // (orignal + scale_direction) length
    if transform.scale.length() == (Vec3::X + Vec3::Y + scale_direction).length() {
        audio_event.send(AudioEvent::new(&audio_assets.ding, false));
    }
}
