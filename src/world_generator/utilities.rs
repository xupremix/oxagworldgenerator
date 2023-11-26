use std::ops::RangeInclusive;

use rand::{thread_rng, rngs::StdRng, SeedableRng, Rng};

/// Returns a randomly generated seed
pub fn generate_random_seed() -> u64 {
    thread_rng().gen::<u64>()
}

/// Returs a randomly generated world size
pub fn generate_random_world_size(seed: u64) -> usize {
    StdRng::seed_from_u64(seed).gen_range(0..u8::MAX) as usize
}

/// Returns true if `range1` contains `range2` (start and end included), false otherwise
pub(crate) fn contains(range1: &RangeInclusive<f64>, range2: &RangeInclusive<f64>) -> bool {
    (range1.start() <= range2.start()) && (range1.end() >= range2.end())
}