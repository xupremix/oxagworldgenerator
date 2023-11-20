use robotics_lib::world::tile::Content;
use std::ops::RangeInclusive;

pub const DEFAULT_WORLD_SIZE: usize = 256;
pub const IN_BATCH_PROBABILITY: f64 = 0.5;
pub const PRESENT_PROBABILITY: f64 = 0.5;
pub const DEFAULT_MIN_SPAWN_NUMBER: usize = 0;
pub const MAP_RANGE: RangeInclusive<f64> = -1.0..=1.0;

// noise default values
pub const DEFAULT_NOISE_DIM: usize = 2;
pub const DEFAULT_NOISE_SEED: u32 = 42;
pub const DEFAULT_NOISE_X: usize = 256;
pub const DEFAULT_NOISE_Y: usize = 256;
pub const DEFAULT_NOISE_SEAMLESS: bool = true;
pub const DEFAULT_NOISE_XY_BOUND1: f64 = -1.0;
pub const DEFAULT_NOISE_XY_BOUND2: f64 = 1.0;
pub const DEFAULT_NOISE_FREQUENCY: f64 = 2.5;
pub const DEFAULT_NOISE_LACUNARITY: f64 = 2.0;
pub const DEFAULT_NOISE_PERSISTANCE: f64 = 0.6;
pub const DEFAULT_NOISE_OCTAVES: usize = 12;

#[derive(Debug)]
pub enum OxAgError {
    InvalidWorldGenerationOption,
    InvalidContentGenerationOption(Content),
    ContentOptionNotSet(Content),
}

pub(crate) fn contains(range1: &RangeInclusive<f64>, range2: &RangeInclusive<f64>) -> bool {
    (range1.start() <= range2.start()) && (range1.end() >= range2.end())
}
