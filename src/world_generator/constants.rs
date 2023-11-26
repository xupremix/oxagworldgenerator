use std::ops::RangeInclusive;

pub const DEFAULT_WORLD_SIZE: usize = 256;
pub const DEFAULT_TILE_CONTENT_IN_BATCH_PROBABILITY: f64 = 0.5;
pub const DEFAULT_TILE_CONTENT_IS_PRESENT_PROBABILITY: f64 = 0.5;
pub const DEFAULT_WEATHER_PROBABILITY: f64 = 0.7;
pub const DEFAULT_TILE_CONTENT_MIN_SPAWN_NUMBER: usize = 0;
pub(crate) const DEFAULT_SPAWN_RANGE_BOUNDS: RangeInclusive<f64> = -1.0..=1.0;

// noise default values
pub(crate) const DEFAULT_NOISE_DIM: usize = 2;
pub(crate) const DEFAULT_NOISE_SEED: u64 = 42;
pub(crate) const DEFAULT_NOISE_X: usize = 256;
pub(crate) const DEFAULT_NOISE_Y: usize = 256;
pub(crate) const DEFAULT_NOISE_SEAMLESS: bool = true;
pub(crate) const DEFAULT_NOISE_XY_LOWER_BOUND: f64 = -1.0;
pub(crate) const DEFAULT_NOISE_XY_UPPER_BOUND: f64 = 1.0;

// TODO: Should we let users configure these?
pub(crate) const DEFAULT_NOISE_FREQUENCY: f64 = 2.5;
pub(crate) const DEFAULT_NOISE_LACUNARITY: f64 = 2.0;
pub(crate) const DEFAULT_NOISE_PERSISTANCE: f64 = 0.6;
pub(crate) const DEFAULT_NOISE_OCTAVES: usize = 12;
