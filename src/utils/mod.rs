use robotics_lib::world::tile::Content;
use std::ops::RangeInclusive;

pub const DEFAULT_WORLD_SIZE: usize = 256;
pub const IN_BATCH_PROBABILITY: f64 = 0.5;
pub const PRESENT_PROBABILITY: f64 = 0.5;
pub const DEFAULT_MIN_SPAWN_NUMBER: usize = 0;
pub const MAP_RANGE: RangeInclusive<f64> = -1.0..=1.0;

#[derive(Debug)]
pub enum OxAgError {
    InvalidWorldGenerationOption,
    InvalidContentGenerationOption(Content),
}

pub(crate) fn contains(range1: &RangeInclusive<f64>, range2: &RangeInclusive<f64>) -> bool {
    (range1.start() <= range2.start()) && (range1.end() >= range2.end())
}
