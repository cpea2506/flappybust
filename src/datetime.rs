use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    random, Rng,
};

#[derive(Resource, Clone, Copy)]
pub enum DateTime {
    Day,
    Night,
}

impl Distribution<DateTime> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DateTime {
        if rng.gen() {
            DateTime::Day
        } else {
            DateTime::Night
        }
    }
}

impl DateTime {
    pub fn gen(mut commands: Commands) {
        let datetime = random::<DateTime>();

        commands.insert_resource(datetime);
    }

    pub fn raw_value(self) -> &'static str {
        match self {
            DateTime::Day => "day",
            DateTime::Night => "night",
        }
    }
}
