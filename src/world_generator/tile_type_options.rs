use std::ops::RangeInclusive;

use rand::prelude::StdRng;
use rand::Rng;
use rand::SeedableRng;

use crate::utils::constants::*;
use crate::utils::errors::OxAgError;
use crate::utils::traits::{Container, FromSeed};

#[derive(Debug, Clone)]
pub struct OxAgTileTypeOptions {
    pub deep_water_level: RangeInclusive<f64>,
    pub shallow_water_level: RangeInclusive<f64>,
    pub sand_level: RangeInclusive<f64>,
    pub grass_level: RangeInclusive<f64>,
    pub hill_level: RangeInclusive<f64>,
    pub mountain_level: RangeInclusive<f64>,
    pub snow_level: RangeInclusive<f64>,
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

        Self {
            deep_water_level: -1.0..=dw_end,
            shallow_water_level: dw_end..=sw_end,
            sand_level: sw_end..=sd_end,
            grass_level: sd_end..=gr_end,
            hill_level: gr_end..=hl_end,
            mountain_level: hl_end..=mt_end,
            snow_level: mt_end..=1.0,
        }
    }
}

impl OxAgTileTypeOptions {
    /// Validates this spawn levels to make sure they are within bounds.
    ///
    /// Returns a [OxAgError] [Result] if validation fails.
    pub fn validate(&self) -> Result<(), OxAgError> {
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