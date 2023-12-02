use std::ops::RangeInclusive;

pub mod constants;
pub mod errors;

/// trait to check if some data is within some other data
pub trait Container<C> {
    fn within(&self, range: &C) -> bool;
}

impl Container<RangeInclusive<f64>> for RangeInclusive<f64> {
    fn within(&self, range: &RangeInclusive<f64>) -> bool {
        (range.start() <= self.start()) && (range.end() >= self.end())
    }
}

use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
use std::io::Write;

/// Returns a randomly generated seed
pub fn generate_random_seed() -> u64 {
    thread_rng().gen::<u64>()
}

/// Returs a randomly generated world size
pub fn generate_random_world_size(seed: u64) -> usize {
    StdRng::seed_from_u64(seed).gen_range(0..u8::MAX) as usize
}

pub(crate) fn progress_bar(
    iteration: usize,
    total: usize,
    prefix: &str,
    length: usize,
    fill: &str,
) {
    let total = total - 1;
    let fill_len = length * iteration / total;
    let out = iteration == total;
    let (suffix, termination) = if out {
        ("\n", "Complete ✔️")
    } else {
        ("\r", "Building...")
    };
    let bar = format!(
        "{} |{}{}| {:.1}% {}{}",
        prefix,
        fill.repeat(fill_len),
        "-".repeat(length - fill_len),
        100.0 * (iteration as f64 / total as f64),
        termination,
        suffix
    );
    print!("{bar}");
}
