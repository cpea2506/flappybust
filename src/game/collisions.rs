use crate::GameState;
use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use flappybust::BasicMath;
use itertools::Itertools;

use super::{
    audio::{AudioAssets, AudioEvent},
    base::Base,
    bird::{components::Bird, events::DeathEvent},
    pipe::Pipe,
};

#[derive(Event, Default)]
pub struct CollisionEvent;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CollisionEvent>().add_systems(
            Update,
            (
                check_collision.run_if(not(in_state(GameState::Ready))),
                on_collision.run_if(in_state(GameState::Playing)),
            ),
        );
    }
}

fn check_collision(
    mut bird: Query<(&mut Transform, &Bird)>,
    pipes: Query<(&Transform, &Pipe), Without<Bird>>,
    bases: Query<&Base, (Without<Pipe>, Without<Bird>)>,
    game_state: Res<State<GameState>>,
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

    // Check pipe collision only on playing state
    // to prevent each frame checking when bird falls inside a pipe.
    if matches!(game_state.get(), GameState::Playing) {
        let bird_collide = |b_pos: Vec3, b_size: Vec2| {
            let bird_bounding_box =
                Aabb2d::new(bird_transform.translation.truncate(), bird.size.half());
            let other_bounding_box = Aabb2d::new(b_pos.truncate(), b_size.half());

            bird_bounding_box.intersects(&other_bounding_box)
        };

        // Collide with pipe.
        for ((pipe_transform, pipe), (flipped_pipe_transform, flipped_pipe)) in
            pipes.iter().tuples()
        {
            let pipe_collision = bird_collide(pipe_transform.translation, pipe.size);
            let flipped_pipe_collision =
                bird_collide(flipped_pipe_transform.translation, flipped_pipe.size);

            if pipe_collision || flipped_pipe_collision {
                collision_event.send_default();
            }
        }
    }
}

fn on_collision(
    mut next_state: ResMut<NextState<GameState>>,
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

    next_state.set(GameState::Over);
}
