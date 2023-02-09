pub mod components;
pub mod events;

use components::*;
use flappybust::Math;
use iyes_loopless::state::CurrentState;

use events::*;

use super::base::Base;
use super::game_over::events::*;
use super::{audio::*, resources::BouncingState, GameState};
use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem};
use rand::random;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeathEvent>()
            .add_enter_system(GameState::Ready, spawn)
            .add_system(flap.run_not_in_state(GameState::Over))
            .add_system(fly.run_not_in_state(GameState::Ready))
            .init_resource::<BouncingState>()
            .add_system(bouncing_y.run_in_state(GameState::Ready))
            .add_enter_system(GameState::Over, death_bird_spawn)
            .add_system(death_bird_fly.run_in_state(GameState::Over));
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

fn death_bird_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    bird: Query<&Bird>,
    base: Query<&Base>,
) {
    let bird = bird.single();
    let base = base.iter().next().expect("base must be initialized first");

    let death_bird_translation = Vec3::new(
        bird.translation.x,
        base.collider_pos + bird.size.y.half(),
        0.5,
    );

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/bird_soul.png"),
            visibility: Visibility::INVISIBLE,
            transform: Transform::from_translation(death_bird_translation),
            ..default()
        },
        BirdSoul {
            translation: death_bird_translation,
        },
    ));
}

fn death_bird_fly(
    mut death_bird: Query<(&mut Transform, &mut Visibility, &BirdSoul)>,
    mut audio_event: EventWriter<AudioEvent>,
    audio_assets: Res<AudioAssets>,
    medal_event: EventReader<MedalDisplayed>,
    mut bird_to_the_heaven_event: EventWriter<BirdToTheHeaven>,
) {
    if medal_event.is_empty() {
        return;
    }

    let (mut transform, mut visibility, death_bird) = death_bird.single_mut();

    visibility.is_visible = true;

    transform.translation.y += 1.;

    if transform.translation.y == death_bird.translation.y + 1. {
        audio_event.send(AudioEvent::new(&audio_assets.heaven, false));
    }

    if transform.translation.y >= 267. {
        bird_to_the_heaven_event.send_default();
    }
}

fn fly(
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    game_state: Res<CurrentState<GameState>>,
    mut bird: Query<(&mut Bird, &mut Transform)>,
    mut audio_event: EventWriter<AudioEvent>,
    audio_assets: Res<AudioAssets>,
    death_event: EventReader<DeathEvent>,
) {
    let (mut bird, mut bird_transform) = bird.single_mut();

    // Rotate down at min -90deg and rotate up at max 25deg
    bird.rotation = (bird.rotation - 40f32.recip()).clamp(-90f32.to_radians(), 25f32.to_radians());

    if game_state.0 == GameState::Playing {
        if keys.just_pressed(KeyCode::Space) || buttons.just_pressed(MouseButton::Left) {
            audio_event.send(AudioEvent::new(&audio_assets.wing, false));

            bird.velocity = -2.35;

            // rotate bird a 25deg angle
            bird.rotation = 25f32.to_radians();
        }

        if keys.just_released(KeyCode::Space) || buttons.just_released(MouseButton::Left) {
            audio_event.send(AudioEvent::new(&audio_assets.swoosh, false));
        }
    }

    // don't try to fall if bird's already dead and
    // lay on the ground
    if !death_event.is_empty() {
        return;
    }

    bird.velocity += bird.gravity;

    bird_transform.translation.y -= bird.velocity;
    bird_transform.rotation = Quat::from_rotation_z(bird.rotation);
}

fn flap(time: Res<Time>, mut bird: Query<(&mut FlapAnimation, &mut Handle<Image>), With<Bird>>) {
    let (mut animation, mut texture) = bird.single_mut();

    animation.timer.tick(time.delta());

    if animation.timer.just_finished() {
        animation.current_frame = (animation.current_frame + 1) % 3;
        *texture = animation.frames[animation.current_frame].clone();
    }
}

fn bouncing_y(
    mut commands: Commands,
    mut bird: Query<(&mut Transform, &Bird)>,
    bouncing_state: Res<BouncingState>,
) {
    let (mut bird_transform, bird) = bird.single_mut();

    let bouncing_radius = 3.;

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
