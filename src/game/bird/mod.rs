automod::dir!(pub "src/game/bird");

use super::{
    AudioAssets, GameState, ImageAssets, audio::messages::AudioMessage, base::components::Base,
    game_over::messages::MedalDisplayed, layer,
};
use bevy::prelude::*;
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
                    (
                        fall.run_if(not(in_state(GameState::Ready))),
                        flap.run_if(not(in_state(GameState::Over))),
                    )
                        .run_if(not(in_state(GameState::AssetLoading))),
                    // Flap before falling so a press moves the bird up this frame, not the next.
                    fly.run_if(in_state(GameState::Playing)).before(fall),
                    bird_soul_fly.run_if(in_state(GameState::Over)),
                ),
            )
            .add_systems(OnEnter(GameState::Over), bird_soul_spawn)
            .add_systems(
                OnExit(GameState::Over),
                (despawn::<Bird>, despawn::<BirdSoul>),
            );
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
    base: Query<&Base>,
    image_assets: Res<ImageAssets>,
) {
    let base = base.iter().next().expect("Base must be initialized first.");
    let translation = Vec3::new(
        bird.translation.x,
        base.collider_pos + Bird::HEIGHT / 2.0,
        layer::BIRD_SOUL,
    );

    commands.spawn((
        Sprite::from_image(image_assets.bird_soul.clone()),
        Visibility::Hidden,
        Transform::from_translation(translation),
        BirdSoul,
    ));
}

fn bird_soul_fly(
    time: Res<Time>,
    bird_soul: Single<(&mut Transform, &mut Visibility), With<BirdSoul>>,
    mut audio_message: MessageWriter<AudioMessage>,
    audio_assets: Res<AudioAssets>,
    medal_message: MessageReader<MedalDisplayed>,
    mut in_heaven: MessageWriter<InTheHeaven>,
) {
    if medal_message.is_empty() {
        return;
    }

    let (mut transform, mut visibility) = bird_soul.into_inner();

    if visibility.set_if_neq(Visibility::Visible) {
        audio_message.write(AudioMessage::new(&audio_assets.heaven, false));
    }

    transform.translation.y += frames(&time);

    if transform.translation.y >= BirdSoul::HEAVEN_Y {
        in_heaven.write_default();
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
