use bevy::{prelude::*, sprite::collide_aabb::collide};
use flappybust::Math;
use itertools::Itertools;
use iyes_loopless::prelude::*;

use crate::GameState;

use super::{
    audio::{AudioAssets, AudioEvent},
    base::Base,
    bird::{components::Bird, events::DeathEvent},
    pipe::Pipe,
};

#[derive(Default)]
pub struct CollisionEvent;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CollisionEvent>()
            .add_system(check_collision.run_not_in_state(GameState::Ready))
            .add_system(on_collision.run_in_state(GameState::Playing));
    }
}

fn check_collision(
    mut bird: Query<(&mut Transform, &Bird)>,
    pipes: Query<(&Transform, &Pipe), Without<Bird>>,
    bases: Query<&Base, (Without<Bird>, Without<Pipe>)>,
    game_state: Res<CurrentState<GameState>>,
    mut collision_event: EventWriter<CollisionEvent>,
    mut death_event: EventWriter<DeathEvent>,
) {
    let (mut bird_transform, bird) = bird.single_mut();

    // there are two bases (for animate purpose) but we only need to take one
    // because bird only collides with the top of any base
    let base = bases.iter().next().expect("base must be initialized first");

    // check if bird bottom collides with top base
    if bird_transform.translation.y - bird.size.y.half() <= base.collider_pos {
        // this is for bird to lay on the ground
        bird_transform.translation.y = base.collider_pos + bird.size.y.half();

        death_event.send_default();
        collision_event.send_default();
    }

    // check pipe collision only on playing state
    // to prevent each frame check when bird falls inside a pipe
    if game_state.0 == GameState::Playing {
        let bird_collide = |b_pos: Vec3, b_size: Vec2| {
            collide(bird_transform.translation, bird.size, b_pos, b_size)
        };

        // collapsed with pipe
        for ((pipe_transform, pipe), (flipped_pipe_transform, flipped_pipe)) in
            pipes.iter().tuples()
        {
            let pipe_collision = bird_collide(pipe_transform.translation, pipe.size);
            let flipped_pipe_collision =
                bird_collide(flipped_pipe_transform.translation, flipped_pipe.size);

            if pipe_collision.is_some() || flipped_pipe_collision.is_some() {
                collision_event.send_default();
            }
        }
    }
}

fn on_collision(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    mut audio_event: EventWriter<AudioEvent>,
    collision_event: EventReader<CollisionEvent>,
) {
    if collision_event.is_empty() {
        return;
    }

    audio_event.send_batch(vec![
        AudioEvent::new(&audio_assets.die, false),
        AudioEvent::new(&audio_assets.hit, false),
    ]);

    commands.insert_resource(NextState(GameState::Over));
}
