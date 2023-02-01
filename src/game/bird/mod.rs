pub mod components;
pub mod events;

use components::{Bird, BirdColor, FlapAnimation};
use iyes_loopless::state::CurrentState;

use events::DeathEvent;

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
            .add_system(bouncing_y.run_in_state(GameState::Ready));
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bird_color = random::<BirdColor>();
    let animation_frames = ["up", "mid", "down"]
        .map(|state| asset_server.load(format!("images/bird_{bird_color}_{state}.png")));
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

    if game_state.0 == GameState::Playing {
        if keys.just_pressed(KeyCode::Space) || buttons.just_pressed(MouseButton::Left) {
            audio_event.send(AudioEvent::new(&audio_assets.wing, false));

            bird.velocity = -2.35
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
        commands.insert_resource(BouncingState::DOWN);
    } else if distance <= -bouncing_radius {
        commands.insert_resource(BouncingState::UP);
    }

    match bouncing_state.into_inner() {
        BouncingState::UP => {
            bird_transform.translation.y += 0.5;
        }
        BouncingState::DOWN => {
            bird_transform.translation.y -= 0.5;
        }
    }
}
