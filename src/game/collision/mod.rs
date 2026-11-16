automod::dir!("src/game/collision");

use super::{
    AudioAssets,
    audio::messages::AudioMessage,
    base::components::Base,
    bird::{components::Bird, messages::DeathMessage},
    pipe::components::Pipe,
};
use crate::GameState;
use bevy::{
    prelude::*,
    shape::{Aabb2d, IntersectsVolume},
    transform::TransformSystems,
};
use messages::CollisionMessage;

/// Collision logic.
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // Run after all `Update` movement so collisions use this frame's positions,
        // but before transforms propagate so landing on the base renders this frame.
        app.add_message::<CollisionMessage>().add_systems(
            PostUpdate,
            (
                check_collision
                    .run_if(not(in_state(GameState::Ready)))
                    .run_if(not(in_state(GameState::AssetLoading))),
                on_collision.run_if(in_state(GameState::Playing)),
            )
                .chain()
                .before(TransformSystems::Propagate),
        );
    }
}

fn check_collision(
    mut bird_transform: Single<&mut Transform, With<Bird>>,
    pipes: Query<&Transform, (With<Pipe>, Without<Bird>)>,
    bases: Query<&Base, (Without<Pipe>, Without<Bird>)>,
    game_state: Res<State<GameState>>,
    mut collision_message: MessageWriter<CollisionMessage>,
    mut death_message: MessageWriter<DeathMessage>,
) {
    // There are two bases (for animating purpose) but we only need to take one
    // because bird only collides with the top of any base.
    let base = bases
        .iter()
        .next()
        .expect("Base must be initialized first.");

    // Check if bird bottom collides with top base.
    if bird_transform.translation.y - Bird::HEIGHT / 2.0 <= base.collider_pos {
        // this is for bird to lay on the ground
        bird_transform.translation.y = base.collider_pos + Bird::HEIGHT / 2.0;

        death_message.write_default();
        collision_message.write_default();
    }

    // Check pipe collision only on playing state
    // to prevent each frame checking when bird falls inside a pipe.
    if *game_state.get() == GameState::Playing {
        let bird_box = Aabb2d::new(
            bird_transform.translation.truncate(),
            Vec2::new(Bird::WIDTH, Bird::HEIGHT) / 2.0,
        );
        let pipe_half_size = Vec2::new(Pipe::WIDTH, Pipe::HEIGHT) / 2.0;

        if pipes.iter().any(|pipe| {
            bird_box.intersects(&Aabb2d::new(pipe.translation.truncate(), pipe_half_size))
        }) {
            collision_message.write_default();
        }
    }
}

fn on_collision(
    mut next_state: ResMut<NextState<GameState>>,
    audio_assets: Res<AudioAssets>,
    mut audio_message: MessageWriter<AudioMessage>,
    collision_message: MessageReader<CollisionMessage>,
) {
    if collision_message.is_empty() {
        return;
    }

    audio_message.write_batch(vec![
        AudioMessage::new(&audio_assets.die, false),
        AudioMessage::new(&audio_assets.hit, false),
    ]);

    next_state.set(GameState::Over);
}
