use bevy::prelude::Resource;
use flappybust::ternary;
use rand::{distributions::Standard, prelude::Distribution, random, Rng};
use std::fmt::{Display, Formatter, Result};

#[derive(Resource, Default)]
pub enum BouncingState {
    #[default]
    UP,
    DOWN,
}

#[derive(Resource, Clone, Copy)]
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

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            DateTime::Day => "day",
            DateTime::Night => "night",
        })
    }
}
