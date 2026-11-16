automod::dir!(pub "src/game/bird");

use super::{
    AudioAssets, GameState, ImageAssets, audio::messages::AudioMessage, base::components::Base,
    layer, pipe::components::Pipe,
};
use crate::{GameOverPhase, InRound};
use bevy::{
    prelude::*,
    shape::{Aabb2d, IntersectsVolume},
    transform::TransformSystems,
};
use components::*;
use flappybust::{despawn, frames};
use messages::*;
use rand::random_range;

/// Bird logic.
pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DeathMessage>()
            .add_systems(OnEnter(GameState::Ready), spawn)
            .add_systems(
                Update,
                (
                    bounce_vertical.run_if(in_state(GameState::Ready)),
                    fall.run_if(in_state(InRound)),
                    flap.run_if(in_state(GameState::Ready).or_else(in_state(GameState::Playing))),
                    // Flap before falling so a press moves the bird up this frame, not the next.
                    fly.run_if(in_state(GameState::Playing)).before(fall),
                    bird_soul_fly.run_if(in_state(GameOverPhase::Soul)),
                ),
            )
            // After all `Update` movement so it uses this frame's positions,
            // but before transforms propagate so landing on the base renders this frame.
            .add_systems(
                PostUpdate,
                collide
                    .run_if(in_state(InRound))
                    .before(TransformSystems::Propagate),
            )
            .add_systems(OnEnter(GameOverPhase::Soul), bird_soul_spawn)
            .add_systems(OnExit(GameState::Over), despawn::<Bird>);
    }
}

fn spawn(mut commands: Commands, image_assets: Res<ImageAssets>) {
    let birds = [
        &image_assets.red_birds,
        &image_assets.blue_birds,
        &image_assets.yellow_birds,
    ];
    let animation_frames = birds[random_range(0..birds.len())].clone();
    let bird = Bird::default();

    commands.spawn((
        bird,
        Sprite::from_image(animation_frames[0].clone()),
        Transform::from_translation(bird.translation),
        FlapAnimation::new(Bird::FLAP_FRAME_SECS, animation_frames),
        BouncingAnimation::default(),
    ));
}

fn bird_soul_spawn(
    mut commands: Commands,
    bird: Single<&Bird>,
    image_assets: Res<ImageAssets>,
    audio_assets: Res<AudioAssets>,
    mut audio_message: MessageWriter<AudioMessage>,
) {
    let translation = Vec3::new(
        bird.translation.x,
        Base::TOP + Bird::HEIGHT / 2.0,
        layer::BIRD_SOUL,
    );

    commands.spawn((
        Sprite::from_image(image_assets.bird_soul.clone()),
        Transform::from_translation(translation),
        BirdSoul,
        DespawnOnExit(GameState::Over),
    ));
    audio_message.write(AudioMessage::new(&audio_assets.heaven, false));
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

fn fall(
    time: Res<Time>,
    bird: Single<(&mut Bird, &mut Transform)>,
    death_message: MessageReader<DeathMessage>,
) {
    let (mut bird, mut bird_transform) = bird.into_inner();
    let frames = frames(&time);

    bird.rotation = (bird.rotation - Bird::ROTATION_SPEED * frames)
        .clamp(Bird::MIN_ROTATION, Bird::MAX_ROTATION);

    // Don't try to fall if bird has been already dead,
    // just lay on the ground.
    if !death_message.is_empty() {
        return;
    }

    bird.velocity = (bird.velocity + bird.gravity * frames).min(Bird::MAX_VELOCITY);

    bird_transform.translation.y -= bird.velocity * frames;
    bird_transform.rotation = Quat::from_rotation_z(bird.rotation);
}

fn fly(
    mut bird: Single<&mut Bird>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    audio_assets: Res<AudioAssets>,
    mut audio_message: MessageWriter<AudioMessage>,
) {
    if keys.just_pressed(KeyCode::Space)
        || buttons.just_pressed(MouseButton::Left)
        || touches.any_just_pressed()
    {
        audio_message.write(AudioMessage::new(&audio_assets.wing, false));

        bird.velocity = Bird::DEFAULT_VELOCITY;
        bird.rotation = Bird::MAX_ROTATION;
    }

    if keys.just_released(KeyCode::Space)
        || buttons.just_released(MouseButton::Left)
        || touches.any_just_released()
    {
        audio_message.write(AudioMessage::new(&audio_assets.swoosh, false));
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

fn bounce_vertical(time: Res<Time>, bird: Single<(&mut Transform, &mut BouncingAnimation, &Bird)>) {
    let (mut transform, mut bouncing_animation, bird) = bird.into_inner();

    bouncing_animation.phase += BouncingAnimation::SPEED * frames(&time);

    // Set the position from the phase instead of accumulating steps, so it never drifts.
    transform.translation.y =
        bird.translation.y + (1f32 - bouncing_animation.phase.cos()) / BouncingAnimation::SPEED;
}

/// End the game when the bird hits a pipe or the base, and keep a dead bird lying on the base.
fn collide(
    mut bird: Single<&mut Transform, With<Bird>>,
    pipes: Query<&Transform, (With<Pipe>, Without<Bird>)>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    audio_assets: Res<AudioAssets>,
    mut audio_message: MessageWriter<AudioMessage>,
    mut death_message: MessageWriter<DeathMessage>,
) {
    let on_base = bird.translation.y - Bird::HEIGHT / 2.0 <= Base::TOP;

    if on_base {
        bird.translation.y = Base::TOP + Bird::HEIGHT / 2.0;
        death_message.write_default();
    }

    // Only while playing, so a bird falling through a pipe after dying doesn't hit it again.
    if *state.get() != GameState::Playing {
        return;
    }

    let bird_box = Aabb2d::new(
        bird.translation.truncate(),
        Vec2::new(Bird::WIDTH, Bird::HEIGHT) / 2.0,
    );
    let pipe_half_size = Vec2::new(Pipe::WIDTH, Pipe::HEIGHT) / 2.0;
    let hit_pipe = pipes
        .iter()
        .any(|pipe| bird_box.intersects(&Aabb2d::new(pipe.translation.truncate(), pipe_half_size)));

    if on_base || hit_pipe {
        audio_message.write_batch([
            AudioMessage::new(&audio_assets.die, false),
            AudioMessage::new(&audio_assets.hit, false),
        ]);
        next_state.set(GameState::Over);
    }
}
