use std::fmt::Display;

use bevy::prelude::*;
use flappybust::ternary;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Resource, Clone, Copy)]
pub enum DateTime {
    Day,
    Night,
}

impl Distribution<DateTime> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DateTime {
        ternary!(rng.gen(), DateTime::Day, DateTime::Night)
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DateTime::Day => "day",
            DateTime::Night => "night",
        })
    }
}
