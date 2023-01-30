use bevy::prelude::*;

pub fn despawn_all(mut commands: Commands, entities: Query<Entity>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

pub fn despawn<T: Component>(mut commands: Commands, entity: Query<Entity, With<T>>) {
    commands.entity(entity.single()).despawn();
}
