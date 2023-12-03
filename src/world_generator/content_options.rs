use rand::prelude::StdRng;
use rand::Rng;
use rand::SeedableRng;
use robotics_lib::world::tile::Content;
use strum::IntoEnumIterator;

use crate::utils::constants::*;

/// Options that determine how the tile [Content] are spawned
/// TODO: Examples
#[derive(Debug, Copy, Clone)]
pub struct OxAgContentOptions {
    /// Whether this [Content] should be spawned in sets or groups
    pub in_batches: bool,
    /// Whether this [Content] should be present or not
    pub is_present: bool,
    pub min_spawn_number: usize,
    pub max_radius: usize,
    pub with_max_spawn_number: bool,
    pub max_spawn_number: usize,
}

impl Default for OxAgContentOptions {
    fn default() -> Self {
        Self {
            in_batches: false,
            is_present: false,
            min_spawn_number: 0,
            max_radius: 0,
            with_max_spawn_number: true,
            max_spawn_number: 0,
        }
    }
}

impl OxAgContentOptions {
    /// Creates a new [OxAgContentOptions], calculating it from a given `seed`.
    pub fn new_from_seed(seed: u64, size: usize) -> Vec<(Content, Self)> {
        let mut rng = StdRng::seed_from_u64(seed);

        Content::iter()
            .filter_map(|content: Content| match content {
                Content::None => None,
                other => {
                    let max_radius = rng.gen_range(1..DEFAULT_SPAWN_RADIUS);
                    Some((
                        other,
                        Self {
                            in_batches: rng.gen_bool(DEFAULT_TILE_CONTENT_IN_BATCH_PROBABILITY),
                            is_present: rng.gen_bool(DEFAULT_TILE_CONTENT_IS_PRESENT_PROBABILITY),
                            min_spawn_number: DEFAULT_TILE_CONTENT_MIN_SPAWN_NUMBER,
                            max_radius,
                            with_max_spawn_number: rng.gen_bool(0.5),
                            max_spawn_number: rng.gen_range(
                                0..(size.pow(2) / (max_radius.pow(2) as f64 * 3.14) as usize
                                    + DEFAULT_BATCH_DISTANCE),
                            ),
                        },
                    ))
                }
            })
            .collect()
    }
}
