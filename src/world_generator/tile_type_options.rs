use std::ops::RangeInclusive;

use crate::utils::constants::*;
use crate::utils::errors::OxAgError;
use crate::utils::traits::{Container, FromSeed, Validator};
use rand::prelude::StdRng;
use rand::Rng;
use rand::SeedableRng;

#[derive(Debug, Clone)]
/// Options that determine how the tile [Tile] are spawned
/// TODO: Examples?
pub struct OxAgTileTypeOptions {
    /// The range inclusive area where you want the Deep Water [Tile] to spawn
    pub deep_water_level: RangeInclusive<f64>,
    /// The range inclusive area where you want the Shallow Water [Tile] to spawn
    pub shallow_water_level: RangeInclusive<f64>,
    /// The range inclusive area where you want the Sand [Tile] to spawn
    pub sand_level: RangeInclusive<f64>,
    /// The range inclusive area where you want the Grass [Tile] to spawn
    pub grass_level: RangeInclusive<f64>,
    /// The range inclusive area where you want the Hill [Tile] to spawn
    pub hill_level: RangeInclusive<f64>,
    /// The range inclusive area where you want the Mountain [Tile] to spawn
    pub mountain_level: RangeInclusive<f64>,
    /// The range inclusive area where you want the Snow [Tile] to spawn
    pub snow_level: RangeInclusive<f64>,
    /// The number of River you want to spawn
    pub river_n: RangeInclusive<usize>,
    /// The number of Street [Tile] you want to spawn
    pub street_n: RangeInclusive<usize>,
    /// The length of a Street [Tile]
    pub street_len: RangeInclusive<usize>,
    /// The number of Lava lake that you want to spawn
    pub lava_n: RangeInclusive<usize>,
    /// The radius of the Lava lake
    pub lava_radius: RangeInclusive<usize>,
}

impl FromSeed for OxAgTileTypeOptions {
    fn new_from_seed(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);

        let dw_end = rng.gen_range(DEFAULT_SPAWN_RANGE_BOUNDS);
        let sw_end = rng.gen_range(dw_end..=1.0);
        let sd_end = rng.gen_range(sw_end..=1.0);
        let gr_end = rng.gen_range(sd_end..=1.0);
        let hl_end = rng.gen_range(gr_end..=1.0);
        let mt_end = rng.gen_range(hl_end..=1.0);
        let r_max = rng.gen_range(1..64);
        let r_max = rng.gen_range(0..r_max);
        let s_max = rng.gen_range(1..64);
        let s_max = rng.gen_range(0..s_max);
        let l_max = rng.gen_range(1..30);
        let l_max = rng.gen_range(0..l_max);
        let lr_max = rng.gen_range(1..8);
        let lr_max = rng.gen_range(0..lr_max);
        let st_max = rng.gen_range(1..20);
        let st_max = rng.gen_range(0..st_max);

        Self {
            deep_water_level: -1.0..=dw_end,
            shallow_water_level: dw_end..=sw_end,
            sand_level: sw_end..=sd_end,
            grass_level: sd_end..=gr_end,
            hill_level: gr_end..=hl_end,
            mountain_level: hl_end..=mt_end,
            snow_level: mt_end..=1.0,
            river_n: 0..=r_max,
            street_n: 0..=s_max,
            street_len: 0..=st_max,
            lava_n: 0..=l_max,
            lava_radius: 0..=lr_max,
        }
    }
}

impl Validator for OxAgTileTypeOptions {
    /// Validates this spawn levels to make sure they are within bounds.
    ///
    /// Returns a [OxAgError] [Result] if validation fails.
    fn validate(&self) -> Result<(), OxAgError> {
        let levels = [
            &self.deep_water_level,
            &self.shallow_water_level,
            &self.sand_level,
            &self.grass_level,
            &self.hill_level,
            &self.mountain_level,
            &self.snow_level,
        ];
        // at least one with start = -1.0
        if !levels.iter().any(|l| *l.start() == -1.0) {
            Err(OxAgError::WrongLowerBound)?
        }

        // at least one with end = 1.0
        if !levels.iter().any(|l| *l.end() == 1.0) {
            Err(OxAgError::WrongUpperBound)?
        }

        // all must be between -1.0 and 1.0        }
        if levels
            .iter()
            .any(|l| !l.within(&DEFAULT_SPAWN_RANGE_BOUNDS))
        {
            Err(OxAgError::RangesAreOutOfBounds)?
        }

        Ok(())
    }
}
