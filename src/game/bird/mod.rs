automod::dir!(pub "src/game/bird");

use super::{
    audio::events::AudioEvent, base::components::Base, game_over::events::MedalDisplayed,
    AudioAssets, GameState, ImageAssets,
};
use bevy::prelude::*;
use components::*;
use events::*;
use flappybust::{despawn, BasicMath, Switcher};
use rand::random;

/// Bird logic.
pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeathEvent>()
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
                    fly.run_if(in_state(GameState::Playing)),
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
    let bird_color = random::<BirdColor>();
    let animation_frames = match bird_color {
        BirdColor::Red => image_assets.red_birds.clone(),
        BirdColor::Blue => image_assets.blue_birds.clone(),
        BirdColor::Yellow => image_assets.yellow_birds.clone(),
    };
    let bird = Bird::new(-53f32, 9f32);

    commands.spawn((
        bird,
        SpriteBundle {
            texture: animation_frames[0].clone(),
            transform: Transform::from_translation(bird.translation),
            ..default()
        },
        FlapAnimation::new(0.12, animation_frames),
        BouncingAnimation::default(),
    ));
}

fn bird_soul_spawn(
    mut commands: Commands,
    bird: Query<&Bird>,
    base: Query<&Base>,
    image_assets: Res<ImageAssets>,
) {
    let bird = bird.single();
    let base = base.iter().next().expect("Base must be initialized first.");
    let translation = Vec3::new(
        bird.translation.x,
        base.collider_pos + Bird::HEIGHT.half(),
        0.5,
    );

    commands.spawn((
        SpriteBundle {
            texture: image_assets.bird_soul.clone(),
            visibility: Visibility::Hidden,
            transform: Transform::from_translation(translation),
            ..default()
        },
        BirdSoul { translation },
    ));
}

fn bird_soul_fly(
    mut bird_soul: Query<(&mut Transform, &mut Visibility, &BirdSoul)>,
    mut audio_event: EventWriter<AudioEvent>,
    audio_assets: Res<AudioAssets>,
    medal_event: EventReader<MedalDisplayed>,
    mut in_heaven: EventWriter<InTheHeaven>,
) {
    if medal_event.is_empty() {
        return;
    }

    let (mut transform, mut visibility, bird_soul) = bird_soul.single_mut();

    if matches!(*visibility, Visibility::Hidden) {
        visibility.on();
    }

    transform.translation.y += 1f32;

    if transform.translation.y == bird_soul.translation.y + 1f32 {
        audio_event.send(AudioEvent::new(&audio_assets.heaven, false));
    }

    if transform.translation.y >= 267f32 {
        in_heaven.send_default();
    }
}

fn fall(mut bird: Query<(&mut Bird, &mut Transform)>, death_event: EventReader<DeathEvent>) {
    let (mut bird, mut bird_transform) = bird.single_mut();

    // Rotate down at min -90deg and rotate up at max 25deg.
    bird.rotation = (bird.rotation - 40f32.recip()).clamp(-90f32.to_radians(), 25f32.to_radians());

    // Don't try to fall if bird has been already dead,
    // just lay on the ground.
    if !death_event.is_empty() {
        return;
    }

    bird.velocity += bird.gravity;

    bird_transform.translation.y -= bird.velocity;
    bird_transform.rotation = Quat::from_rotation_z(bird.rotation);
}

fn fly(
    mut bird: Query<&mut Bird>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    audio_assets: Res<AudioAssets>,
    mut audio_event: EventWriter<AudioEvent>,
) {
    let mut bird = bird.single_mut();

    if keys.just_pressed(KeyCode::Space) || buttons.just_pressed(MouseButton::Left) {
        audio_event.send(AudioEvent::new(&audio_assets.wing, false));

        bird.velocity = Bird::DEFAULT_VELOCITY;
        bird.rotation = 25f32.to_radians();
    }

    if keys.just_released(KeyCode::Space) || buttons.just_released(MouseButton::Left) {
        audio_event.send(AudioEvent::new(&audio_assets.swoosh, false));
    }
}

fn flap(time: Res<Time>, mut bird: Query<(&mut FlapAnimation, &mut Handle<Image>), With<Bird>>) {
    let (mut animation, mut texture) = bird.single_mut();

    animation.timer.tick(time.delta());

    if animation.timer.just_finished() {
        animation.current_frame = (animation.current_frame + 1) % 3;
        *texture = animation.frames[animation.current_frame].clone();
    }
}

fn bounce_vertical(mut bird: Query<(&mut Transform, &mut BouncingAnimation), With<Bird>>) {
    let (mut transform, mut bouncing_animation) = bird.single_mut();

    transform.translation.y += bouncing_animation.velocity.sin();
    bouncing_animation.velocity += 0.16;
}
