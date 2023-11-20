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

#[derive(Debug)]
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
        let mut rng = StdRng::seed_from_u64(seed as u64);

        Content::iter()
            .map(|content: Content| {
                (
                    content,
                    Self {
                        in_batches: rng.gen_bool(IN_BATCH_PROBABILITY),
                        present: rng.gen_bool(PRESENT_PROBABILITY),
                        min_spawn_number: DEFAULT_MIN_SPAWN_NUMBER,
                        spawn_level: rng.gen_range(MAP_RANGE),
                    },
                )
            })
            .collect::<HashMap<Content, Self>>()
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
        // implementation
        HashMap::new()
    };
}

// if we want to use flow fields for setting water content and street tiles
// struct Flow {
//     x: isize,
//     y: isize,
// }
