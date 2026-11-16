automod::dir!(pub "src/game/bird");

use super::{
    AudioAssets, GameState, ImageAssets, layer,
    pipe::components::{Coin, Pipe},
};
use crate::GameOverPhase;
use bevy::{
    prelude::*,
    shape::{Aabb2d, BoundingCircle, IntersectsVolume},
};
use components::*;
use flappybust::{Position, flap_pressed, frames, sound};
use rand::random_range;

/// Bird logic.
pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        // Systems that need the bird skip themselves while it doesn't exist (e.g. while loading).
        app.add_systems(OnEnter(GameState::Ready), spawn)
            .add_systems(
                FixedUpdate,
                (
                    bounce_vertical.run_if(in_state(GameState::Ready)),
                    fall.run_if(not(in_state(GameState::Ready))),
                ),
            )
            // After all `FixedUpdate` movement so it uses this step's positions.
            .add_systems(
                FixedPostUpdate,
                (collide, collect_coin).run_if(in_state(GameState::Playing)),
            )
            // Before the fixed steps so a press moves the bird up this frame, not the next.
            .add_systems(
                RunFixedMainLoop,
                tilt.in_set(RunFixedMainLoopSystems::AfterFixedMainLoop)
                    .run_if(not(in_state(GameState::Ready))),
            )
            .add_systems(
                RunFixedMainLoop,
                fly.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop)
                    .run_if(in_state(GameState::Playing))
                    .run_if(flap_pressed),
            )
            .add_systems(
                Update,
                (
                    swoosh.run_if(in_state(GameState::Playing)),
                    flap.run_if(not(in_state(GameState::Over))),
                    bird_soul_fly.run_if(in_state(GameOverPhase::Soul)),
                ),
            )
            .add_systems(OnEnter(GameState::Over), play_crash)
            .add_systems(OnEnter(GameOverPhase::Soul), bird_soul_spawn);
    }
}

fn spawn(mut commands: Commands, image_assets: Res<ImageAssets>) {
    let birds = [
        &image_assets.red_birds,
        &image_assets.blue_birds,
        &image_assets.yellow_birds,
    ];
    let animation_frames = birds[random_range(0..birds.len())].clone();

    commands.spawn((
        Bird::default(),
        Sprite::from_image(animation_frames[0].clone()),
        Position(Bird::START.extend(layer::BIRD)),
        FlapAnimation::new(Bird::FLAP_FRAME_SECS, animation_frames),
        BouncingAnimation::default(),
        DespawnOnExit(GameState::Over),
    ));
}

fn bird_soul_spawn(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    audio_assets: Res<AudioAssets>,
) {
    commands.spawn((
        Sprite::from_image(image_assets.bird_soul.clone()),
        Transform::from_xyz(Bird::START.x, Bird::GROUND_Y, layer::BIRD_SOUL),
        BirdSoul,
        DespawnOnExit(GameState::Over),
    ));
    commands.spawn((
        sound(&audio_assets.heaven),
        DespawnOnExit(GameOverPhase::Soul),
    ));
}

fn bird_soul_fly(
    time: Res<Time>,
    mut bird_soul: Single<&mut Transform, With<BirdSoul>>,
    mut next_phase: ResMut<NextState<GameOverPhase>>,
) {
    bird_soul.translation.y += frames(&time);

    if bird_soul.translation.y >= BirdSoul::HEAVEN_Y {
        next_phase.set(GameOverPhase::Restart);
    }
}

fn fall(bird: Single<(&mut Bird, &mut Position)>) {
    let (mut bird, mut position) = bird.into_inner();

    bird.previous_rotation = bird.rotation;

    // A dead bird just lies on the base.
    if position.y <= Bird::GROUND_Y {
        return;
    }

    if bird.flap_queued {
        bird.flap_queued = false;
        bird.velocity = Bird::DEFAULT_VELOCITY;
        bird.rotation = Bird::FLAP_ROTATION;
    }

    bird.rotation = (bird.rotation - Bird::ROTATION_SPEED).max(Bird::MIN_ROTATION);
    bird.velocity = (bird.velocity + Bird::GRAVITY).min(Bird::MAX_VELOCITY);

    position.y = (position.y - bird.velocity).max(Bird::GROUND_Y);
}

/// Show the tilt between the last two fixed steps, like `Position` is shown.
fn tilt(time: Res<Time<Fixed>>, bird: Single<(&Bird, &mut Transform)>) {
    let (bird, mut transform) = bird.into_inner();
    let rotation = bird
        .previous_rotation
        .lerp(bird.rotation, time.overstep_fraction());

    transform.rotation = Quat::from_rotation_z(rotation.min(Bird::MAX_ROTATION));
}

/// Queue a flap for the next fixed step, which may not run this frame.
fn fly(mut commands: Commands, mut bird: Single<&mut Bird>, audio_assets: Res<AudioAssets>) {
    commands.spawn(sound(&audio_assets.wing));
    bird.flap_queued = true;
}

fn swoosh(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    audio_assets: Res<AudioAssets>,
) {
    if keys.just_released(KeyCode::Space)
        || buttons.just_released(MouseButton::Left)
        || touches.any_just_released()
    {
        commands.spawn(sound(&audio_assets.swoosh));
    }
}

fn flap(time: Res<Time>, bird: Single<(&mut FlapAnimation, &mut Sprite), With<Bird>>) {
    let (mut animation, mut sprite) = bird.into_inner();

    animation.timer.tick(time.delta());

    if animation.timer.just_finished() {
        animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
        sprite.image = animation.frames[animation.current_frame].clone();
    }
}

fn bounce_vertical(bird: Single<(&mut Position, &mut BouncingAnimation), With<Bird>>) {
    let (mut position, mut bouncing_animation) = bird.into_inner();

    bouncing_animation.phase += BouncingAnimation::SPEED;

    // Set the position from the phase instead of accumulating steps, so it never drifts.
    position.y = Bird::START.y + (1f32 - bouncing_animation.phase.cos()) / BouncingAnimation::SPEED;
}

/// End the game when the bird hits a pipe or lands on the base.
fn collide(
    bird: Single<&Position, With<Bird>>,
    pipes: Query<&Position, With<Pipe>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let bird_circle = bird_circle(&bird);
    let pipe_half_size = Vec2::new(Pipe::WIDTH, Pipe::HEIGHT) / 2.0;
    let hit_pipe = pipes
        .iter()
        .any(|pipe| bird_circle.intersects(&Aabb2d::new(pipe.truncate(), pipe_half_size)));

    if bird.y <= Bird::GROUND_Y || hit_pipe {
        next_state.set(GameState::Over);
    }
}

/// On entering game over, not in `collide`, which may run again before the state changes.
fn play_crash(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn(sound(&audio_assets.die));
    commands.spawn(sound(&audio_assets.hit));
}

/// Remove coins the bird touches.
fn collect_coin(
    mut commands: Commands,
    bird: Single<&Position, With<Bird>>,
    coins: Query<(Entity, &Position), With<Coin>>,
    audio_assets: Res<AudioAssets>,
) {
    let bird_circle = bird_circle(&bird);

    for (coin, position) in &coins {
        if bird_circle.intersects(&BoundingCircle::new(position.truncate(), Coin::SIZE / 2.0)) {
            commands.entity(coin).despawn();
            commands.spawn((
                AudioPlayer::new(audio_assets.ding.clone()),
                PlaybackSettings::DESPAWN.with_speed(1.5),
            ));
        }
    }
}

/// A circle fits the round bird at any rotation, unlike a box.
fn bird_circle(position: &Position) -> BoundingCircle {
    BoundingCircle::new(position.truncate(), Bird::HEIGHT / 2.0)
}
