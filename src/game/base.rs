use crate::GameState;
use bevy::prelude::*;
use flappybust::{ternary, BasicMath};

#[derive(Component, Default)]
pub struct Base {
    pub size: Vec2,

    translation: Vec3,
    secondary: bool,
    pub collider_pos: f32,
}

impl Base {
    pub const WIDTH: f32 = 336f32;
    pub const HEIGHT: f32 = 112f32;
    const RESET_POINT: f32 = Base::WIDTH - 24f32;

    fn new(x: f32, y: f32, secondary: bool) -> Self {
        Base {
            translation: Vec3::new(x, y, 0.4),
            size: Vec2::new(Self::WIDTH, Self::HEIGHT),
            secondary,
            collider_pos: y + Self::HEIGHT.half(),
        }
    }

    fn generate_bundle(self, texture: &Handle<Image>) -> (SpriteBundle, Self) {
        (
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_translation(self.translation),
                ..default()
            },
            self,
        )
    }
}

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Ready), spawn)
            .add_systems(Update, moving.run_if(not(in_state(GameState::Over))));
    }
}

fn spawn(mut commands: Commands, window: Query<&Window>, asset_server: Res<AssetServer>) {
    if let Ok(window) = window.get_single() {
        let base = Base::new(
            0f32,
            Base::HEIGHT.half() - window.resolution.height().half(),
            false,
        );
        let secondary_base = Base::new(Base::WIDTH, base.translation.y, true);
        let texture = asset_server.load("images/base.png");

        commands.spawn_batch(vec![
            base.generate_bundle(&texture),
            secondary_base.generate_bundle(&texture),
        ]);
    }
}

fn moving(mut base: Query<(&mut Base, &mut Transform)>) {
    for (mut base, mut transform) in &mut base {
        base.translation.x = (base.translation.x - 1.5f32) % Base::RESET_POINT;

        transform.translation.x =
            base.translation.x + ternary!(base.secondary, Base::RESET_POINT, 0f32);
    }
}
