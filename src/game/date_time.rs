use bevy::prelude::Resource;
use flappybust::ternary;
use rand::{distributions::Standard, prelude::Distribution, random, Rng};
use strum::AsRefStr;

/// Represent time of a day in game.
#[derive(AsRefStr, Clone, Copy, Resource)]
#[strum(serialize_all = "lowercase")]
pub enum DateTime {
    Day,
    Night,
}

impl Distribution<DateTime> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DateTime {
        ternary!(rng.gen_bool(0.5), DateTime::Day, DateTime::Night)
    }
}

impl Default for DateTime {
    fn default() -> Self {
        random::<Self>()
    }
}
