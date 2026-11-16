automod::dir!(pub "src/game/game_over");

use super::{AudioAssets, ImageAssets, bird::components::Bird, layer, score::components::Score};
use crate::{GameOverPhase, GameState};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use components::*;
use flappybust::{Position, frames, sound};

/// Game over logic.
pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Over), spawn_game_over)
            .add_systems(
                FixedUpdate,
                (bounce_game_over_text, move_scoreboard_up, finish_board)
                    .chain()
                    .run_if(in_state(GameOverPhase::Board)),
            )
            .add_systems(OnEnter(GameOverPhase::Medal), spawn_medal)
            .add_systems(Update, scale_medal.run_if(in_state(GameOverPhase::Medal)))
            .add_systems(OnEnter(GameOverPhase::Restart), spawn_restart_btn)
            .add_systems(
                Update,
                restart_game
                    .run_if(in_state(GameOverPhase::Restart))
                    .run_if(input_just_pressed(KeyCode::Space)),
            );
    }
}

fn spawn_game_over(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        Sprite::from_image(image_assets.game_over.clone()),
        Position(Vec3::new(0f32, GameOverText::START_Y, layer::HUD)),
        GameOverText::default(),
        DespawnOnExit(GameState::Over),
    ));

    commands.spawn((
        Sprite::from_image(image_assets.scoreboard.clone()),
        Position(Vec3::new(0f32, Scoreboard::START_Y, layer::HUD)),
        Scoreboard::default(),
        DespawnOnExit(GameState::Over),
    ));
}

fn bounce_game_over_text(game_over_text: Single<(&mut Position, &mut GameOverText)>) {
    let (mut position, mut game_over_text) = game_over_text.into_inner();

    game_over_text.velocity += GameOverText::GRAVITY;
    position.y -= game_over_text.velocity;

    if position.y < GameOverText::REST_Y {
        if game_over_text.bounced {
            position.y = GameOverText::REST_Y;
        } else {
            game_over_text.velocity *= -GameOverText::BOUNCINESS;
            game_over_text.bounced = true;
        }
    }
}

fn move_scoreboard_up(scoreboard: Single<(&mut Position, &mut Scoreboard)>) {
    let (mut position, mut scoreboard) = scoreboard.into_inner();

    scoreboard.velocity += Scoreboard::GRAVITY;

    position.y = (position.y + scoreboard.velocity).clamp(Scoreboard::START_Y, Scoreboard::END_Y);
}

/// Move on to the medal once the text and scoreboard are in place and the bird lies on the base.
fn finish_board(
    game_over_text: Single<(&Position, &GameOverText)>,
    scoreboard: Single<&Position, With<Scoreboard>>,
    bird: Single<&Position, With<Bird>>,
    mut next_phase: ResMut<NextState<GameOverPhase>>,
) {
    let (text_position, game_over_text) = *game_over_text;

    if game_over_text.bounced
        && text_position.y == GameOverText::REST_Y
        && scoreboard.y == Scoreboard::END_Y
        && bird.y == Bird::GROUND_Y
    {
        next_phase.set(GameOverPhase::Medal);
    }
}

fn spawn_medal(
    mut commands: Commands,
    score: Res<Score>,
    image_assets: Res<ImageAssets>,
    mut next_phase: ResMut<NextState<GameOverPhase>>,
) {
    let image = match score.current {
        0..10 => {
            // No medal for low scores.
            next_phase.set(GameOverPhase::Soul);
            return;
        }
        10..20 => &image_assets.bronze_medal,
        20..30 => &image_assets.silver_medal,
        30..40 => &image_assets.gold_medal,
        _ => &image_assets.platinum_medal,
    };

    commands.spawn((
        Sprite::from_image(image.clone()),
        Transform::from_translation(Medal::POSITION.extend(layer::MEDAL))
            .with_scale(Vec2::splat(Medal::START_SCALE).extend(1f32)),
        Medal,
        DespawnOnExit(GameState::Over),
    ));
}

fn scale_medal(
    mut commands: Commands,
    time: Res<Time>,
    audio_assets: Res<AudioAssets>,
    mut medal: Single<&mut Transform, With<Medal>>,
    mut next_phase: ResMut<NextState<GameOverPhase>>,
) {
    // Shrink both xy back to the original size.
    let scale = (medal.scale.x - Medal::SHRINK_SPEED * frames(&time)).max(1f32);
    medal.scale = Vec3::new(scale, scale, 1f32);

    if scale == 1f32 {
        commands.spawn(sound(&audio_assets.ding));
        next_phase.set(GameOverPhase::Soul);
    }
}

fn spawn_restart_btn(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands
        .spawn((
            Sprite::from_image(image_assets.restart_btn.clone()),
            Transform::from_xyz(0f32, RestartButton::Y, layer::HUD),
            RestartButton,
            DespawnOnExit(GameState::Over),
        ))
        // Left click or touch, like the restart press it replaces.
        .observe(
            |press: On<PointerPress>, next_state: ResMut<NextState<GameState>>| {
                if press.button == PointerButton::Primary {
                    restart_game(next_state);
                }
            },
        );
}

fn restart_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Ready);
}
