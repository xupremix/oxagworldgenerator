use rand::prelude::StdRng;
use rand::Rng;
use rand::SeedableRng;
use robotics_lib::world::tile::Content;
use strum::IntoEnumIterator;

use crate::utils::constants::*;
use crate::utils::errors::OxAgError;
use crate::utils::errors::OxAgError::{InvalidContentOption, InvalidContentOptionProvided};
use crate::utils::traits::Validator;

/// Options that determine how the tile [Content] are spawned
/// TODO: Examples & check if doc is ok
#[derive(Debug, Copy, Clone)]
pub struct OxAgContentOptions {
    /// Whether this [Content] should be spawned in sets or groups
    pub in_batches: bool,
    /// Whether this [Content] should be present or not
    pub is_present: bool,
    /// The minimum number of this [Content] present in the world
    pub min_spawn_number: usize,
    /// The max radius used for the spawning in batches of this [Content]
    pub max_radius: usize,
    /// Whether this [Content] has a maximum spawn number
    pub with_max_spawn_number: bool,
    /// The max number of this [Content] present in the world
    pub max_spawn_number: usize,
    /// The total percentage of this [Content] present in the world
    pub percentage: f64,
}

impl Validator for OxAgContentOptions {
    fn validate(&self) -> Result<(), OxAgError> {
        if !CONTENT_PERCENTAGE_RANGE.contains(&self.percentage) {
            Err(InvalidContentOptionProvided)
        } else {
            Ok(())
        }
    }
}

impl Validator for Vec<(Content, OxAgContentOptions)> {
    fn validate(&self) -> Result<(), OxAgError> {
        let mut out = Content::None;
        if self.iter().any(|(c, op)| {
            out = c.to_default();
            !CONTENT_PERCENTAGE_RANGE.contains(&op.percentage)
        }) {
            Err(InvalidContentOption(out))
        } else {
            Ok(())
        }
    }
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
            percentage: 1.0,
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
                            percentage: rng.gen_range(0.0..1.0),
                        },
                    ))
                }
            })
            .collect()
    }
}
