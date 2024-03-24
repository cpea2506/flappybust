use bevy::ecs::event::Event;

#[derive(Default, Event)]
pub struct DeathEvent;

#[derive(Default, Event)]
pub struct BirdToTheHeaven;
