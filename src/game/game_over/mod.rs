automod::dir!(pub "src/game/game_over");

use super::{
    AudioAssets, ImageAssets,
    audio::messages::AudioMessage,
    bird::messages::{DeathMessage, InTheHeaven},
    score::components::Score,
};
use crate::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use components::*;
use flappybust::{despawn, frames};
use messages::*;

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
                    restart_game,
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

fn spawn_game_over(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        Sprite::from_image(image_assets.game_over.clone()),
        Transform::from_xyz(0f32, 351f32, 0.2),
        GameOverText::default(),
    ));

    commands.spawn((
        Sprite::from_image(image_assets.scoreboard.clone()),
        Transform::from_xyz(0f32, -199f32, 0.2),
        Scoreboard::default(),
    ));

    commands.spawn((
        Sprite::from_image(image_assets.restart_btn.clone()),
        Visibility::Hidden,
        Transform::from_xyz(0f32, -35f32, 0.2),
        RestartButton,
    ));
}

fn display_restart_btn(
    mut visibility: Single<&mut Visibility, With<RestartButton>>,
    in_the_heaven: MessageReader<InTheHeaven>,
    mut restart_btn_message: MessageWriter<RestartButtonDisplayed>,
) {
    if in_the_heaven.is_empty() {
        return;
    }

    visibility.set_if_neq(Visibility::Visible);
    restart_btn_message.write_default();
}

fn bounce_game_over_text(
    time: Res<Time>,
    game_over_text: Single<(&mut Transform, &mut GameOverText)>,
    mut game_over_text_message: MessageWriter<GameOverTextDisplayed>,
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
        game_over_text_message.write_default();
    }
}

fn move_scoreboard_up(
    time: Res<Time>,
    scoreboard: Single<(&mut Transform, &mut Scoreboard)>,
    mut scoreboard_message: MessageWriter<ScoreboardDisplayed>,
) {
    let (mut transform, mut scoreboard) = scoreboard.into_inner();

    let frames = frames(&time);

    scoreboard.velocity += scoreboard.gravity * frames;

    transform.translation.y =
        (transform.translation.y + scoreboard.velocity * frames).clamp(-199f32, 57f32);

    if transform.translation.y == 57f32 {
        scoreboard_message.write_default();
    }
}

fn spawn_medal(mut commands: Commands, score: Res<Score>, image_assets: Res<ImageAssets>) {
    let image = match score.current {
        0..10 => return,
        10..20 => &image_assets.bronze_medal,
        20..30 => &image_assets.silver_medal,
        30..40 => &image_assets.gold_medal,
        _ => &image_assets.platinum_medal,
    };

    commands.spawn((
        Sprite::from_image(image.clone()),
        Transform::from_xyz(-65f32, 47f32, 0.4).with_scale(Vec3::new(25f32, 25f32, 1f32)),
        Visibility::Hidden,
        Medal,
    ));
}

#[allow(clippy::too_many_arguments)]
fn scale_medal(
    time: Res<Time>,
    audio_assets: Res<AudioAssets>,
    medal: Option<Single<(&mut Transform, &mut Visibility), With<Medal>>>,
    mut audio_message: MessageWriter<AudioMessage>,
    mut medal_message: MessageWriter<MedalDisplayed>,
    game_over_text_displayed: MessageReader<GameOverTextDisplayed>,
    death_message: MessageReader<DeathMessage>,
    scoreboard_displayed: MessageReader<ScoreboardDisplayed>,
) {
    if scoreboard_displayed.is_empty()
        || death_message.is_empty()
        || game_over_text_displayed.is_empty()
    {
        return;
    }

    // No medal for low scores.
    let Some(medal) = medal else {
        medal_message.write_default();
        return;
    };
    let (mut transform, mut visibility) = medal.into_inner();

    visibility.set_if_neq(Visibility::Visible);

    // Reach the final end position.
    if transform.scale.x <= 1f32 {
        medal_message.write_default();
        return;
    }

    // Shrink both xy back to the original size.
    let scale = (transform.scale.x - 2f32 * frames(&time)).max(1f32);
    transform.scale = Vec3::new(scale, scale, 1f32);

    if scale == 1f32 {
        audio_message.write(AudioMessage::new(&audio_assets.ding, false));
    }
}

#[allow(clippy::too_many_arguments)]
fn restart_game(
    key: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut next_state: ResMut<NextState<GameState>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    restart_btn: Single<&Transform, With<RestartButton>>,
    restart_btn_displayed: MessageReader<RestartButtonDisplayed>,
) {
    if restart_btn_displayed.is_empty() {
        return;
    }

    let (camera, camera_transform) = *camera;
    let btn_area = Rect::from_center_size(restart_btn.translation.truncate(), RestartButton::SIZE);
    // Where the mouse was clicked or a finger touched down this frame.
    let mut presses = window
        .cursor_position()
        .filter(|_| mouse.just_pressed(MouseButton::Left))
        .into_iter()
        .chain(touches.iter_just_pressed().map(|touch| touch.position()));
    let clicked_btn = presses.any(|press| {
        camera
            .viewport_to_world_2d(camera_transform, press)
            .is_ok_and(|press| btn_area.contains(press))
    });

    if key.just_pressed(KeyCode::Space) || clicked_btn {
        next_state.set(GameState::Ready);
    }
}
