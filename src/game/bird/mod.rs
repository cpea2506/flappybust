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
use flappybust::{flap_pressed, frames, sound};
use rand::random_range;

/// Bird logic.
pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        // Systems that need the bird skip themselves while it doesn't exist (e.g. while loading).
        app.add_systems(OnEnter(GameState::Ready), spawn)
            .add_systems(
                Update,
                (
                    bounce_vertical.run_if(in_state(GameState::Ready)),
                    fall.run_if(not(in_state(GameState::Ready))),
                    flap.run_if(not(in_state(GameState::Over))),
                    // Flap before falling so a press moves the bird up this frame, not the next.
                    fly.run_if(in_state(GameState::Playing))
                        .run_if(flap_pressed)
                        .before(fall),
                    swoosh.run_if(in_state(GameState::Playing)),
                    bird_soul_fly.run_if(in_state(GameOverPhase::Soul)),
                ),
            )
            // After all `Update` movement so it uses this frame's positions.
            .add_systems(PostUpdate, collide.run_if(in_state(GameState::Playing)))
            .add_systems(
                PostUpdate,
                collect_coin.run_if(in_state(GameState::Playing)),
            )
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
        Transform::from_translation(Bird::START.extend(layer::BIRD)),
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

fn fall(time: Res<Time>, bird: Single<(&mut Bird, &mut Transform)>) {
    let (mut bird, mut transform) = bird.into_inner();

    // A dead bird just lies on the base.
    if transform.translation.y <= Bird::GROUND_Y {
        return;
    }

    let frames = frames(&time);

    bird.rotation = (bird.rotation - Bird::ROTATION_SPEED * frames).max(Bird::MIN_ROTATION);
    bird.velocity = (bird.velocity + Bird::GRAVITY * frames).min(Bird::MAX_VELOCITY);

    transform.translation.y =
        (transform.translation.y - bird.velocity * frames).max(Bird::GROUND_Y);
    transform.rotation = Quat::from_rotation_z(bird.rotation.min(Bird::MAX_ROTATION));
}

fn fly(mut commands: Commands, mut bird: Single<&mut Bird>, audio_assets: Res<AudioAssets>) {
    commands.spawn(sound(&audio_assets.wing));

    bird.velocity = Bird::DEFAULT_VELOCITY;
    bird.rotation = Bird::FLAP_ROTATION;
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

fn bounce_vertical(
    time: Res<Time>,
    bird: Single<(&mut Transform, &mut BouncingAnimation), With<Bird>>,
) {
    let (mut transform, mut bouncing_animation) = bird.into_inner();

    bouncing_animation.phase += BouncingAnimation::SPEED * frames(&time);

    // Set the position from the phase instead of accumulating steps, so it never drifts.
    transform.translation.y =
        Bird::START.y + (1f32 - bouncing_animation.phase.cos()) / BouncingAnimation::SPEED;
}

/// End the game when the bird hits a pipe or lands on the base.
fn collide(
    mut commands: Commands,
    bird: Single<&Transform, With<Bird>>,
    pipes: Query<&Transform, (With<Pipe>, Without<Bird>)>,
    mut next_state: ResMut<NextState<GameState>>,
    audio_assets: Res<AudioAssets>,
) {
    let bird_circle = bird_circle(&bird);
    let pipe_half_size = Vec2::new(Pipe::WIDTH, Pipe::HEIGHT) / 2.0;
    let hit_pipe = pipes.iter().any(|pipe| {
        bird_circle.intersects(&Aabb2d::new(pipe.translation.truncate(), pipe_half_size))
    });

    if bird.translation.y <= Bird::GROUND_Y || hit_pipe {
        commands.spawn(sound(&audio_assets.die));
        commands.spawn(sound(&audio_assets.hit));
        next_state.set(GameState::Over);
    }
}

/// Remove coins the bird touches.
fn collect_coin(
    mut commands: Commands,
    bird: Single<&Transform, With<Bird>>,
    coins: Query<(Entity, &Transform), With<Coin>>,
    audio_assets: Res<AudioAssets>,
) {
    let bird_circle = bird_circle(&bird);

    for (coin, transform) in &coins {
        if bird_circle.intersects(&BoundingCircle::new(
            transform.translation.truncate(),
            Coin::SIZE / 2.0,
        )) {
            commands.entity(coin).despawn();
            commands.spawn((
                AudioPlayer::new(audio_assets.ding.clone()),
                PlaybackSettings::DESPAWN.with_speed(1.5),
            ));
        }
    }
}

/// A circle fits the round bird at any rotation, unlike a box.
fn bird_circle(transform: &Transform) -> BoundingCircle {
    BoundingCircle::new(transform.translation.truncate(), Bird::HEIGHT / 2.0)
}
