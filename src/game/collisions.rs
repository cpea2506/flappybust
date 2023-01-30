use bevy::{prelude::*, sprite::collide_aabb::collide};
use itertools::Itertools;
use iyes_loopless::prelude::*;

use crate::GameState;

use super::{
    audio::{AudioAssets, AudioEvent},
    base::Base,
    bird::Bird,
    pipe::Pipe,
};

#[derive(Default)]
struct CollisionEvent;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CollisionEvent>()
            .add_system(check_collision.run_not_in_state(GameState::Ready))
            .add_system(on_collision.run_in_state(GameState::Playing));
    }
}

fn check_collision(
    bird: Query<(&mut Transform, &Bird)>,
    pipes: Query<(&Transform, &Pipe), Without<Bird>>,
    bases: Query<(&Transform, &Base), (Without<Bird>, Without<Pipe>)>,
    game_state: Res<CurrentState<GameState>>,
    mut collision_event: EventWriter<CollisionEvent>,
) {
    let (bird_transform, bird) = bird.single();

    let bird_collide =
        |b_pos: Vec3, b_size: Vec2| collide(bird_transform.translation, bird.size, b_pos, b_size);

    // there are two bases (for animate purpose) but we only need to take one
    // because bird only collides with the top of any base
    let (base_transform, base) = bases.iter().next().expect("base must be initialized first");

    let base_collision = bird_collide(base_transform.translation, base.size);

    if base_collision.is_some() {
        collision_event.send_default();
    }

    // check pipe collision only on playing state
    // to prevent each frame check when bird falls inside a pipe
    if game_state.0 == GameState::Playing {
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

    audio_event.send(AudioEvent {
        audio: audio_assets.die.clone(),
    });

    commands.insert_resource(NextState(GameState::Over));
}
