use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug)]
enum Nychthemeron {
    Day,
    Night,
}

impl Nychthemeron {
    pub fn raw_value(&self) -> &str {
        match self {
            Nychthemeron::Day => "day",
            Nychthemeron::Night => "night",
        }
    }
}

impl Distribution<Nychthemeron> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Nychthemeron {
        match rng.gen_range(0..2) {
            0 => Nychthemeron::Day,
            _ => Nychthemeron::Night,
        }
    }
}

#[derive(Component, Default)]
pub struct Background {
    pub x: f32,
    pub secondary: bool,
}

impl Background {
    pub fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
        let nychthemeron = rand::random::<Nychthemeron>();

        let texture: Handle<Image> = asset_server.load(&format!(
            "images/background-{}.png",
            nychthemeron.raw_value()
        ));

        commands
            .spawn_bundle(SpriteBundle {
                texture: texture.clone(),
                ..default()
            })
            .insert(Background::default());
        commands
            .spawn_bundle(SpriteBundle {
                texture,
                transform: Transform::from_xyz(288., 0., 0.),
                ..default()
            })
            .insert(Background {
                secondary: true,
                ..default()
            });
    }

    pub fn moving_system(mut query: Query<(&mut Background, &mut Transform)>) {
        for (mut background, mut transform) in query.iter_mut() {
            background.x = (background.x - 1.) % 288.;

            if background.secondary {
                transform.translation.x = background.x + 288.;
            } else {
                transform.translation.x = background.x;
            }
        }
    }
}
