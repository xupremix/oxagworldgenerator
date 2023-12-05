use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};

pub mod constants;
pub mod errors;
pub mod traits;

/// Returns a randomly generated seed
pub fn generate_random_seed() -> u64 {
    thread_rng().gen::<u64>()
}

pub fn multiplier_from_seed(seed: u64) -> f64 {
    let mut rng = StdRng::seed_from_u64(seed);
    rng.gen::<f64>()
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
    if total == 1 {
        println!("Total should never reach 1");
    }
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
