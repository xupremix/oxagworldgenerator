use robotics_lib::world::tile::Content;
use robotics_lib::world::tile::Content::Bank;
use std::ops::RangeInclusive;

pub const DEFAULT_WORLD_SIZE: usize = 256;
pub const IN_BATCH_PROBABILITY: f64 = 0.5;
pub const PRESENT_PROBABILITY: f64 = 0.5;
pub const DEFAULT_MIN_SPAWN_NUMBER: usize = 0;
pub const MAP_RANGE: RangeInclusive<f64> = -1.0..=1.0;

// noise default values
pub const DEFAULT_NOISE_DIM: usize = 2;
pub const DEFAULT_NOISE_SEED: u64 = 42;
pub const DEFAULT_NOISE_X: usize = 256;
pub const DEFAULT_NOISE_Y: usize = 256;
pub const DEFAULT_NOISE_SEAMLESS: bool = true;
pub const DEFAULT_NOISE_XY_BOUND1: f64 = -1.0;
pub const DEFAULT_NOISE_XY_BOUND2: f64 = 1.0;
pub const DEFAULT_NOISE_FREQUENCY: f64 = 2.5;
pub const DEFAULT_NOISE_LACUNARITY: f64 = 2.0;
pub const DEFAULT_NOISE_PERSISTANCE: f64 = 0.6;
pub const DEFAULT_NOISE_OCTAVES: usize = 12;

#[derive(Debug, Clone)]
pub enum OxAgError {
    InvalidWorldGenerationOption,
    InvalidContentGenerationOption(Content),
    ContentOptionNotSet(Content),
}

pub(crate) fn contains(range1: &RangeInclusive<f64>, range2: &RangeInclusive<f64>) -> bool {
    (range1.start() <= range2.start()) && (range1.end() >= range2.end())
}

pub(crate) fn default_from(content: Content) -> Content {
    match content {
        Content::Coin(_) => Content::Coin(0),
        Content::Garbage(_) => Content::Garbage(0),
        Content::Water(_) => Content::Water(0),
        Content::Rock(_) => Content::Rock(0),
        Content::Tree(_) => Content::Tree(0),
        Content::Bin(_) => Content::Bin(0..0),
        Content::Bank(_) => Content::Bank(0..0),
        Content::Crate(_) => Content::Crate(0..0),
        other => other,
    }
}
