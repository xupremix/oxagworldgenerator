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
    pub spawn_level: f64,
}

impl Default for OxAgTileContentSpawnOptions {
    fn default() -> Self {
        Self {
            in_batches: false,
            is_present: false,
            min_spawn_number: 0,
            spawn_level: 0.0,
        }
    }
}

impl OxAgTileContentSpawnOptions {
    /// Validates the spawn level of this content options.
    /// It must be in the [DEFAULT_SPAWN_RANGE_BOUNDS] range.
    ///
    /// Returns a [OxAgError] [Result] if it's invalid.
    pub fn validate(&self, content: &Content) -> Result<(), OxAgError> {
        DEFAULT_SPAWN_RANGE_BOUNDS
            .contains(&self.spawn_level)
            .then_some(())
            .ok_or(OxAgError::InvalidSpawnLevel(content.clone()))?;
        
        Ok(())
    }
    
    /// Creates a new [OxAgTileContentSpawnOptions], calculating it from a given `seed`.
    pub fn new_from_seed(seed: u64) -> HashMap<Content, Self> {
        let mut rng = StdRng::seed_from_u64(seed);

        Content::iter()
            .filter_map(|content: Content| match content {
                Content::None => None,
                other => Some((
                    other,
                    Self {
                        in_batches: rng.gen_bool(DEFAULT_TILE_CONTENT_IN_BATCH_PROBABILITY),
                        is_present: rng.gen_bool(DEFAULT_TILE_CONTENT_IS_PRESENT_PROBABILITY),
                        min_spawn_number: DEFAULT_TILE_CONTENT_MIN_SPAWN_NUMBER,
                        spawn_level: rng.gen_range(DEFAULT_SPAWN_RANGE_BOUNDS),
                    },
                )),
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
                    in_batches: true,
                    is_present: true,
                    min_spawn_number: 2,
                    spawn_level: 0.5,
                },
            ),
            (
                Content::Tree(0),
                OxAgTileContentSpawnOptions {
                    in_batches: false,
                    is_present: true,
                    min_spawn_number: 3,
                    spawn_level: 0.8,
                },
            ),
            (
                Content::Garbage(0),
                OxAgTileContentSpawnOptions {
                    in_batches: true,
                    is_present: true,
                    min_spawn_number: 2,
                    spawn_level: 0.3,
                },
            ),
            (
                Content::Fire,
                OxAgTileContentSpawnOptions {
                    in_batches: true,
                    is_present: true,
                    min_spawn_number: 1,
                    spawn_level: 0.9,
                },
            ),
            (
                Content::Coin(0),
                OxAgTileContentSpawnOptions {
                    in_batches: false,
                    is_present: true,
                    min_spawn_number: 2,
                    spawn_level: 0.6,
                },
            ),
            (
                Content::Bin(0..0),
                OxAgTileContentSpawnOptions {
                    in_batches: false,
                    is_present: true,
                    min_spawn_number: 1,
                    spawn_level: 0.99,
                },
            ),
            (
                Content::Crate(0..0),
                OxAgTileContentSpawnOptions {
                    in_batches: false,
                    is_present: true,
                    min_spawn_number: 1,
                    spawn_level: 0.99,
                },
            ),
            (
                Content::Bank(0..0),
                OxAgTileContentSpawnOptions {
                    in_batches: false,
                    is_present: true,
                    min_spawn_number: 1,
                    spawn_level: 0.99,
                },
            ),
            (
                Content::Water(0),
                OxAgTileContentSpawnOptions {
                    in_batches: true,
                    is_present: true,
                    min_spawn_number: 4,
                    spawn_level: 0.5,
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
