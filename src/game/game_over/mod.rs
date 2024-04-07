automod::dir!(pub "src/game/game_over");

use super::{
    audio::events::AudioEvent,
    bird::events::{DeathEvent, InTheHeaven},
    score::components::Score,
    AudioAssets, ImageAssets,
};
use crate::GameState;
use bevy::prelude::*;
use components::*;
use events::*;
use flappybust::{despawn, Switcher};

/// Game over logic.
pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScoreboardDisplayed>()
            .add_event::<MedalDisplayed>()
            .add_event::<InTheHeaven>()
            .add_event::<RestartButtonDisplayed>()
            .add_event::<GameOverTextDisplayed>()
            .add_systems(OnEnter(GameState::Over), (spawn_game_over, spawn_medal))
            .add_systems(
                Update,
                (
                    scale_medal,
                    move_scoreboard_up,
                    bounce_game_over_text,
                    display_restart_btn,
                )
                    .run_if(in_state(GameState::Over)),
            )
            .add_systems(
                OnExit(GameState::Over),
                (
                    despawn::<Medal>,
                    despawn::<GameOverText>,
                    despawn::<Scoreboard>,
                    despawn::<RestartButton>,
                ),
            );
    }
}

fn spawn_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0f32, 351f32, 0.2),
            texture: asset_server.load("images/game_over.png"),
            ..default()
        },
        GameOverText::default(),
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0f32, -199f32, 0.2),
            texture: asset_server.load("images/scoreboard.png"),
            ..default()
        },
        Scoreboard::default(),
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/restart_btn.png"),
            visibility: Visibility::Hidden,
            transform: Transform::from_xyz(0f32, -35f32, 0.2),
            ..default()
        },
        RestartButton,
    ));
}

fn display_restart_btn(
    mut restart_btn: Query<&mut Visibility, With<RestartButton>>,
    in_the_heaven: EventReader<InTheHeaven>,
    mut restart_btn_event: EventWriter<RestartButtonDisplayed>,
) {
    if in_the_heaven.is_empty() {
        return;
    }

    let mut visibility = restart_btn.single_mut();

    if matches!(*visibility, Visibility::Hidden) {
        visibility.on();
    }

    restart_btn_event.send_default();
}

fn bounce_game_over_text(
    mut game_over_text: Query<(&mut Transform, &mut GameOverText)>,
    mut game_over_text_event: EventWriter<GameOverTextDisplayed>,
) {
    let (mut transform, mut game_over_text) = game_over_text.single_mut();

    game_over_text.velocity += game_over_text.gravity;
    transform.translation.y -= game_over_text.velocity;

    if transform.translation.y < 156f32 {
        if game_over_text.bounce {
            game_over_text.velocity *= -0.73;
            game_over_text.bounce = false;
            return;
        }

        transform.translation.y = 156f32;
        game_over_text_event.send_default();
    }
}

fn move_scoreboard_up(
    mut scoreboard: Query<(&mut Transform, &mut Scoreboard)>,
    mut scoreboard_event: EventWriter<ScoreboardDisplayed>,
) {
    let (mut transform, mut scoreboard) = scoreboard.single_mut();

    scoreboard.velocity += scoreboard.gravity;

    transform.translation.y = (transform.translation.y + scoreboard.velocity).clamp(-199f32, 57f32);

    if transform.translation.y == 57f32 {
        scoreboard_event.send_default();
    }
}

fn spawn_medal(mut commands: Commands, score: Res<Score>, image_assets: Res<ImageAssets>) {
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
                translation: Vec3::new(-65f32, 47f32, 0.4),
                scale: Vec3::new(25f32, 25f32, 0f32),
                ..Transform::IDENTITY
            },
            visibility: Visibility::Hidden,
            texture: match medal_name {
                Some(name) => match name {
                    MedalType::Bronze => image_assets.bronze_medal.clone(),
                    MedalType::Silver => image_assets.silver_medal.clone(),
                    MedalType::Gold => image_assets.gold_medal.clone(),
                    MedalType::Platinum => image_assets.platinum_medal.clone(),
                },
                None => Handle::Weak(AssetId::default()),
            },
            ..default()
        },
        Medal::new(medal_name),
    ));
}

fn scale_medal(
    audio_assets: Res<AudioAssets>,
    mut medal: Query<(&mut Transform, &mut Visibility, &Medal)>,
    mut audio_event: EventWriter<AudioEvent>,
    mut medal_event: EventWriter<MedalDisplayed>,
    game_over_text_displayed: EventReader<GameOverTextDisplayed>,
    death_event: EventReader<DeathEvent>,
    scoreboard_displayed: EventReader<ScoreboardDisplayed>,
) {
    if scoreboard_displayed.is_empty()
        || death_event.is_empty()
        || game_over_text_displayed.is_empty()
    {
        return;
    }

    let (mut transform, mut visibility, medal) = medal.single_mut();

    if medal.get().is_none() {
        medal_event.send_default();
        return;
    }

    if matches!(*visibility, Visibility::Hidden) {
        visibility.on();
    }

    // Scale both xy for circle.
    let scale_direction = Vec3::X + Vec3::Y;
    let unit_length = Vec3::ONE.length();

    // Scale to orignal state.
    transform.scale = (transform.scale - scale_direction).clamp_length_min(unit_length);

    // Play audio if and only if the scale length is reached.
    // (orignal + scale_direction) length.
    if transform.scale == (Vec3::X + Vec3::Y + scale_direction) {
        audio_event.send(AudioEvent::new(&audio_assets.ding, false));
    }

    // Reach the final end position.
    if transform.scale == (transform.scale - scale_direction).clamp_length_min(unit_length) {
        medal_event.send_default();
    }
}
