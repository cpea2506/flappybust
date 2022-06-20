use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug)]
pub enum Nychthemeron {
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
    _x: f32,
}
