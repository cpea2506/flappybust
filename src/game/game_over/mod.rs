automod::dir!(pub "src/game/game_over");

use super::{
    AudioAssets, ImageAssets,
    audio::events::AudioEvent,
    bird::events::{DeathEvent, InTheHeaven},
    score::components::Score,
};
use crate::GameState;
use bevy::prelude::*;
use components::*;
use events::*;
use flappybust::{Switcher, despawn, frames};

/// Game over logic.
pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ScoreboardDisplayed>()
            .add_message::<MedalDisplayed>()
            .add_message::<InTheHeaven>()
            .add_message::<RestartButtonDisplayed>()
            .add_message::<GameOverTextDisplayed>()
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
        Sprite::from_image(asset_server.load("images/game_over.png")),
        Transform::from_xyz(0f32, 351f32, 0.2),
        GameOverText::default(),
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("images/scoreboard.png")),
        Transform::from_xyz(0f32, -199f32, 0.2),
        Scoreboard::default(),
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("images/restart_btn.png")),
        Visibility::Hidden,
        Transform::from_xyz(0f32, -35f32, 0.2),
        RestartButton,
    ));
}

fn display_restart_btn(
    mut visibility: Single<&mut Visibility, With<RestartButton>>,
    in_the_heaven: MessageReader<InTheHeaven>,
    mut restart_btn_event: MessageWriter<RestartButtonDisplayed>,
) {
    if in_the_heaven.is_empty() {
        return;
    }

    if matches!(**visibility, Visibility::Hidden) {
        visibility.on();
    }

    restart_btn_event.write_default();
}

fn bounce_game_over_text(
    time: Res<Time>,
    game_over_text: Single<(&mut Transform, &mut GameOverText)>,
    mut game_over_text_event: MessageWriter<GameOverTextDisplayed>,
) {
    let (mut transform, mut game_over_text) = game_over_text.into_inner();

    let frames = frames(&time);

    game_over_text.velocity += game_over_text.gravity * frames;
    transform.translation.y -= game_over_text.velocity * frames;

    if transform.translation.y < 156f32 {
        if game_over_text.bounce {
            game_over_text.velocity *= -0.73;
            game_over_text.bounce = false;
            return;
        }

        transform.translation.y = 156f32;
        game_over_text_event.write_default();
    }
}

fn move_scoreboard_up(
    time: Res<Time>,
    scoreboard: Single<(&mut Transform, &mut Scoreboard)>,
    mut scoreboard_event: MessageWriter<ScoreboardDisplayed>,
) {
    let (mut transform, mut scoreboard) = scoreboard.into_inner();

    let frames = frames(&time);

    scoreboard.velocity += scoreboard.gravity * frames;

    transform.translation.y =
        (transform.translation.y + scoreboard.velocity * frames).clamp(-199f32, 57f32);

    if transform.translation.y == 57f32 {
        scoreboard_event.write_default();
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
        Sprite::from_image(match medal_name {
            Some(MedalType::Bronze) => image_assets.bronze_medal.clone(),
            Some(MedalType::Silver) => image_assets.silver_medal.clone(),
            Some(MedalType::Gold) => image_assets.gold_medal.clone(),
            Some(MedalType::Platinum) => image_assets.platinum_medal.clone(),
            None => default(),
        }),
        Transform {
            translation: Vec3::new(-65f32, 47f32, 0.4),
            scale: Vec3::new(25f32, 25f32, 0f32),
            ..Transform::IDENTITY
        },
        Visibility::Hidden,
        Medal::new(medal_name),
    ));
}

#[allow(clippy::too_many_arguments)]
fn scale_medal(
    time: Res<Time>,
    audio_assets: Res<AudioAssets>,
    medal: Single<(&mut Transform, &mut Visibility, &Medal)>,
    mut audio_event: MessageWriter<AudioEvent>,
    mut medal_event: MessageWriter<MedalDisplayed>,
    game_over_text_displayed: MessageReader<GameOverTextDisplayed>,
    death_event: MessageReader<DeathEvent>,
    scoreboard_displayed: MessageReader<ScoreboardDisplayed>,
) {
    if scoreboard_displayed.is_empty()
        || death_event.is_empty()
        || game_over_text_displayed.is_empty()
    {
        return;
    }

    let (mut transform, mut visibility, medal) = medal.into_inner();

    if medal.get().is_none() {
        medal_event.write_default();
        return;
    }

    if matches!(*visibility, Visibility::Hidden) {
        visibility.on();
    }

    // Reach the final end position.
    if transform.scale.x <= 1f32 {
        medal_event.write_default();
        return;
    }

    // Shrink both xy back to the original size.
    let scale = (transform.scale.x - frames(&time)).max(1f32);
    transform.scale = Vec3::new(scale, scale, 1f32);

    if scale == 1f32 {
        audio_event.write(AudioEvent::new(&audio_assets.ding, false));
    }
}
