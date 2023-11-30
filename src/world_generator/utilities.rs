use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
use robotics_lib::world::tile::Content;
use robotics_lib::world::tile::Content::{
    Bank, Bin, Coin, Crate, Fire, Fish, Garbage, Market, Rock, Tree, Water,
};
use std::cmp::min;

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
