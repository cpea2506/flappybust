use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Base {
    pub translation: Vec3,
    pub secondary: bool,
}

impl Base {
    fn new(translation: Vec3, secondary: bool) -> Self {
        Base {
            translation,
            secondary,
        }
    }

    pub fn height() -> f32 {
        112.
    }

    pub fn width() -> f32 {
        336.
    }

    pub fn startup_system(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        window: Res<Windows>,
    ) {
        let window = window.get_primary().unwrap();
        let texture = asset_server.load("images/base.png");

        let base = Base::new(
            Vec3::new(0., Base::height() / 2. - window.height() / 2., 0.1),
            false,
        );
        let secondary_base = Base::new(
            Vec3::new(
                Base::width(),
                Base::height() / 2. - window.height() / 2.,
                0.1,
            ),
            true,
        );

        commands
            .spawn_bundle(SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_translation(base.translation),
                ..default()
            })
            .insert(base);
        commands
            .spawn_bundle(SpriteBundle {
                texture,
                transform: Transform::from_translation(secondary_base.translation),
                ..default()
            })
            .insert(secondary_base);
    }

    pub fn moving_system(mut query: Query<(&mut Base, &mut Transform)>) {
        for (mut base, mut transform) in query.iter_mut() {
            base.translation.x = (base.translation.x - 1.) % 312.;

            transform.translation.x = base.translation.x + if base.secondary { 312. } else { 0. };
        }
    }
}
