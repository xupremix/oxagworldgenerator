use std::collections::HashMap;

use rand::prelude::StdRng;
use rand::Rng;
use rand::SeedableRng;
use robotics_lib::world::tile::Content;
use strum::IntoEnumIterator;

use crate::utils::errors::OxAgError;

use super::constants::*;

/// Options that determine how the tile [Content] are spawned
/// TODO: Examples
#[derive(Debug, Copy, Clone)]
pub struct OxAgTileContentSpawnOptions {
    /// Whether this [Content] should be spawned in sets or groups
    pub in_batches: bool,
    /// Whether this [Content] should be present or not
    pub is_present: bool,
    /// TODO: Remove?
    pub min_spawn_number: usize,
    /// TOOD: Range?
    pub max_radius: usize,
    pub with_max_spawn_number: bool,
    pub max_spawn_number: usize,
}

impl Default for OxAgTileContentSpawnOptions {
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

impl OxAgTileContentSpawnOptions {
    /// Creates a new [OxAgTileContentSpawnOptions], calculating it from a given `seed`.
    pub fn new_from_seed(seed: u64, size: usize) -> HashMap<Content, Self> {
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

    /// Creates a new [OxAgTileContentSpawnOptions] from a given `preset`.
    pub fn new_from_preset(preset: OxAgTileContentSpawnOptionPresets) -> HashMap<Content, Self> {
        match preset {
            OxAgTileContentSpawnOptionPresets::DEFAULT => presets::DEFAULT(),
        }
    }
}

pub enum OxAgTileContentSpawnOptionPresets {
    DEFAULT,
    // TODO: More presets
}

pub(crate) mod presets {
    use std::collections::HashMap;

    use robotics_lib::world::tile::Content;

    use crate::world_generator::tile_content_spawn_options::OxAgTileContentSpawnOptions;

    pub const DEFAULT: fn() -> HashMap<Content, OxAgTileContentSpawnOptions> = || {
        HashMap::from([
            (
                Content::Rock(0),
                OxAgTileContentSpawnOptions {
                    in_batches: false,
                    is_present: true,
                    min_spawn_number: 20,
                    max_radius: 0,
                    with_max_spawn_number: false,
                    max_spawn_number: 0,
                },
            ),
            (
                Content::Tree(0),
                OxAgTileContentSpawnOptions {
                    in_batches: true,
                    is_present: true,
                    min_spawn_number: 3,
                    max_radius: 40,
                    with_max_spawn_number: false,
                    max_spawn_number: 100,
                },
            ),
            (
                Content::Garbage(0),
                OxAgTileContentSpawnOptions {
                    in_batches: true,
                    is_present: true,
                    min_spawn_number: 2,
                    max_radius: 0,
                    with_max_spawn_number: false,
                    max_spawn_number: 0,
                },
            ),
            (
                Content::Fire,
                OxAgTileContentSpawnOptions {
                    in_batches: true,
                    is_present: true,
                    min_spawn_number: 10,
                    max_radius: 10,
                    with_max_spawn_number: true,
                    max_spawn_number: 40,
                },
            ),
            (
                Content::Coin(0),
                OxAgTileContentSpawnOptions {
                    in_batches: false,
                    is_present: true,
                    min_spawn_number: 2,
                    max_radius: 3,
                    with_max_spawn_number: false,
                    max_spawn_number: 0,
                },
            ),
            (
                Content::Bin(0..0),
                OxAgTileContentSpawnOptions {
                    in_batches: false,
                    is_present: true,
                    min_spawn_number: 1,
                    max_radius: 3,
                    with_max_spawn_number: false,
                    max_spawn_number: 0,
                },
            ),
            (
                Content::Crate(0..0),
                OxAgTileContentSpawnOptions {
                    in_batches: false,
                    is_present: true,
                    min_spawn_number: 1,
                    max_radius: 1,
                    with_max_spawn_number: false,
                    max_spawn_number: 0,
                },
            ),
            (
                Content::Bank(0..0),
                OxAgTileContentSpawnOptions {
                    in_batches: false,
                    is_present: true,
                    min_spawn_number: 1,
                    max_radius: 3,
                    with_max_spawn_number: false,
                    max_spawn_number: 0,
                },
            ),
            (
                Content::Water(0),
                OxAgTileContentSpawnOptions {
                    in_batches: false,
                    is_present: true,
                    min_spawn_number: 4,
                    max_radius: 1,
                    with_max_spawn_number: false,
                    max_spawn_number: 0,
                },
            ),
            (
                Content::Fish(0),
                OxAgTileContentSpawnOptions {
                    in_batches: false,
                    is_present: true,
                    min_spawn_number: 0,
                    max_radius: 1,
                    with_max_spawn_number: false,
                    max_spawn_number: 0,
                },
            ),
        ])
    };
}

// if we want to use flow fields for setting water content and street tiles
// struct Flow {
//     x: isize,
//     y: isize,
// }
