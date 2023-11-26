

use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};

/// Returns a randomly generated seed
pub fn generate_random_seed() -> u64 {
    thread_rng().gen::<u64>()
}

/// Returs a randomly generated world size
pub fn generate_random_world_size(seed: u64) -> usize {
    StdRng::seed_from_u64(seed).gen_range(0..u8::MAX) as usize
}
