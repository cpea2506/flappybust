use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use flappybust::ternary;
use rand::{distributions::Standard, prelude::Distribution, random, Rng};

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(key = "theme")]
    pub theme: Handle<AudioSource>,
    #[asset(key = "die")]
    pub die: Handle<AudioSource>,
    #[asset(key = "ding")]
    pub ding: Handle<AudioSource>,
    #[asset(key = "heaven")]
    pub heaven: Handle<AudioSource>,
    #[asset(key = "hit")]
    pub hit: Handle<AudioSource>,
    #[asset(key = "score")]
    pub score: Handle<AudioSource>,
    #[asset(key = "swoosh")]
    pub swoosh: Handle<AudioSource>,
    #[asset(key = "wing")]
    pub wing: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(key = "teko_bold")]
    pub teko_bold: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(key = "base")]
    pub base: Handle<Image>,

    // background
    #[asset(key = "bg_day")]
    pub bg_day: Handle<Image>,
    #[asset(key = "bg_night")]
    pub bg_night: Handle<Image>,

    // birds
    #[asset(key = "bird_soul")]
    pub bird_soul: Handle<Image>,

    #[asset(key = "blue_birds", collection(typed))]
    pub blue_birds: Vec<Handle<Image>>,
    #[asset(key = "red_birds", collection(typed))]
    pub red_birds: Vec<Handle<Image>>,
    #[asset(key = "yellow_birds", collection(typed))]
    pub yellow_birds: Vec<Handle<Image>>,

    // pipes
    #[asset(key = "green_pipe")]
    pub green_pipe: Handle<Image>,
    #[asset(key = "red_pipe")]
    pub red_pipe: Handle<Image>,

    #[asset(key = "ready_message")]
    pub ready_message: Handle<Image>,

    #[asset(key = "bronze_medal")]
    pub bronze_medal: Handle<Image>,
    #[asset(key = "silver_medal")]
    pub silver_medal: Handle<Image>,
    #[asset(key = "gold_medal")]
    pub gold_medal: Handle<Image>,
    #[asset(key = "platinum_medal")]
    pub platinum_medal: Handle<Image>,
}

/// Represent time of a day in game.
#[derive(Clone, Copy, Resource, PartialEq)]
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
