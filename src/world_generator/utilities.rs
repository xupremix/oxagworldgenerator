use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
use robotics_lib::world::tile::Content;
use robotics_lib::world::tile::Content::{
    Bank, Bin, Coin, Crate, Fire, Fish, Garbage, Market, Rock, Tree, Water,
};
use std::cmp::min;
use std::fmt::format;
use std::io;
use std::io::Write;

/// Returns a randomly generated seed
pub fn generate_random_seed() -> u64 {
    thread_rng().gen::<u64>()
}

/// Returs a randomly generated world size
pub fn generate_random_world_size(seed: u64) -> usize {
    StdRng::seed_from_u64(seed).gen_range(0..u8::MAX) as usize
}

pub(crate) trait ToValue {
    fn to(&self, value: usize) -> Self;
}

impl ToValue for Content {
    fn to(&self, value: usize) -> Self {
        match self {
            Content::Rock(_) => Rock(value),
            Content::Tree(_) => Tree(value),
            Content::Garbage(_) => Garbage(value),
            Content::Fire => Fire,
            Content::Coin(_) => Coin(value),
            Content::Bin(_) => Bin(0..value),
            Content::Crate(_) => Crate(0..value),
            Content::Bank(_) => Bank(0..value),
            Content::Water(_) => Water(value),
            Content::Market(_) => Market(value),
            Content::Fish(_) => Fish(value),
            Content::None => Content::None,
        }
    }
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
