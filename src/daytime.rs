use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    random, Rng,
};

#[derive(Debug)]
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
    pub fn spawn(mut commands: Commands) {
        let nychthemeron = random::<DateTime>();

        commands.insert_resource(nychthemeron);
    }
}
