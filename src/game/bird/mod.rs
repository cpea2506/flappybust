automod::dir!(pub "src/game/bird");

use bevy_asset_loader::asset_collection::AssetCollectionApp;
use components::*;
use flappybust::{BasicMath, Switcher};
use resources::{BirdAssets, BouncingState};

use events::*;

use super::{
    audio::events::AudioEvent, audio::resources::AudioAssets, base::components::Base,
    game_over::events::MedalDisplayed, GameState,
};
use bevy::prelude::*;
use rand::random;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BouncingState>()
            .init_collection::<BirdAssets>()
            .add_event::<DeathEvent>()
            .add_systems(OnEnter(GameState::Ready), spawn)
            .add_systems(OnEnter(GameState::Over), bird_soul_spawn)
            .add_systems(
                Update,
                (
                    bounce_vertical.run_if(in_state(GameState::Ready)),
                    fall.run_if(not(in_state(GameState::Ready))),
                    fly.run_if(in_state(GameState::Playing)),
                    flap.run_if(not(in_state(GameState::Over))),
                    bird_soul_fly.run_if(in_state(GameState::Over)),
                ),
            );
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bird_color = random::<BirdColor>();
    let animation_frames = ["up", "mid", "down"]
        .map(|state| asset_server.load(format!("images/bird_{}_{state}.png", bird_color.as_ref())));
    let bird = Bird::new(-53., 9.);

    commands.spawn((
        bird,
        SpriteBundle {
            texture: animation_frames[0].clone(), // 0. up, 1. mid, 2. down
            transform: Transform::from_translation(bird.translation),
            ..default()
        },
        FlapAnimation::new(0.15, animation_frames),
    ));
}

fn bird_soul_spawn(
    mut commands: Commands,
    bird: Query<&Bird>,
    base: Query<&Base>,
    bird_assets: Res<BirdAssets>,
) {
    let bird = bird.single();
    let base = base.iter().next().expect("Base must be initialized first.");
    let translation = Vec3::new(
        bird.translation.x,
        base.collider_pos + bird.size.y.half(),
        0.5,
    );

    commands.spawn((
        SpriteBundle {
            texture: bird_assets.bird_soul.clone(),
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

fn bounce_vertical(
    mut commands: Commands,
    mut bird: Query<(&mut Transform, &Bird)>,
    bouncing_state: Res<BouncingState>,
) {
    let (mut bird_transform, bird) = bird.single_mut();

    let bouncing_radius = 3f32;

    let distance = bird_transform.translation.y - bird.translation.y;

    if distance >= bouncing_radius {
        commands.insert_resource(BouncingState::Down);
    } else if distance <= -bouncing_radius {
        commands.insert_resource(BouncingState::Up);
    }

    match bouncing_state.into_inner() {
        BouncingState::Up => {
            bird_transform.translation.y += 0.5;
        }
        BouncingState::Down => {
            bird_transform.translation.y -= 0.5;
        }
    }
}
