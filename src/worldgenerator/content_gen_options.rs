use crate::utils::{
    DEFAULT_MIN_SPAWN_NUMBER, IN_BATCH_PROBABILITY, MAP_RANGE, PRESENT_PROBABILITY,
};
use crate::worldgenerator::world_gen_options::OxAgWorldGenerationPresets;
use rand::prelude::StdRng;
use rand::Rng;
use rand::SeedableRng;
use robotics_lib::world::tile::Content;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use strum::IntoEnumIterator;

#[derive(Debug, Copy, Clone)]
pub struct OxAgContentOption {
    pub in_batches: bool,
    pub present: bool,
    pub min_spawn_number: usize,
    pub spawn_level: f64,
}

impl OxAgContentOption {
    pub fn is_valid(&self) -> bool {
        MAP_RANGE.contains(&self.spawn_level)
    }
    pub fn new(seed: u64) -> HashMap<Content, Self> {
        let mut rng = StdRng::seed_from_u64(seed);

        Content::iter()
            .filter_map(|content: Content| match content {
                Content::None => None,
                other => Some((
                    other,
                    Self {
                        in_batches: rng.gen_bool(IN_BATCH_PROBABILITY),
                        present: rng.gen_bool(PRESENT_PROBABILITY),
                        min_spawn_number: DEFAULT_MIN_SPAWN_NUMBER,
                        spawn_level: rng.gen_range(MAP_RANGE),
                    },
                )),
            })
            .collect()
    }
    pub fn from_preset(preset: OxAgContentGenerationPresets) -> HashMap<Content, Self> {
        match preset {
            OxAgContentGenerationPresets::DEFAULT => presets::DEFAULT(),
        }
    }
}

pub enum OxAgContentGenerationPresets {
    DEFAULT,
    // other
}

pub(crate) mod presets {
    use crate::worldgenerator::content_gen_options::OxAgContentOption;
    use robotics_lib::world::tile::Content;
    use std::collections::HashMap;

    pub const DEFAULT: fn() -> HashMap<Content, OxAgContentOption> = || {
        HashMap::from([
            (
                Content::Rock(0),
                OxAgContentOption {
                    in_batches: true,
                    present: true,
                    min_spawn_number: 2,
                    spawn_level: 0.5,
                },
            ),
            (
                Content::Tree(0),
                OxAgContentOption {
                    in_batches: false,
                    present: true,
                    min_spawn_number: 3,
                    spawn_level: 0.8,
                },
            ),
            (
                Content::Garbage(0),
                OxAgContentOption {
                    in_batches: true,
                    present: true,
                    min_spawn_number: 2,
                    spawn_level: 0.3,
                },
            ),
            (
                Content::Fire,
                OxAgContentOption {
                    in_batches: true,
                    present: true,
                    min_spawn_number: 1,
                    spawn_level: 0.9,
                },
            ),
            (
                Content::Coin(0),
                OxAgContentOption {
                    in_batches: false,
                    present: true,
                    min_spawn_number: 2,
                    spawn_level: 0.6,
                },
            ),
            (
                Content::Bin(0..0),
                OxAgContentOption {
                    in_batches: false,
                    present: true,
                    min_spawn_number: 1,
                    spawn_level: 0.99,
                },
            ),
            (
                Content::Crate(0..0),
                OxAgContentOption {
                    in_batches: false,
                    present: true,
                    min_spawn_number: 1,
                    spawn_level: 0.99,
                },
            ),
            (
                Content::Bank(0..0),
                OxAgContentOption {
                    in_batches: false,
                    present: true,
                    min_spawn_number: 1,
                    spawn_level: 0.99,
                },
            ),
            (
                Content::Water(0),
                OxAgContentOption {
                    in_batches: true,
                    present: true,
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
