use std::ops::RangeInclusive;

use rand::prelude::StdRng;
use rand::Rng;
use rand::SeedableRng;

use crate::utils::OxAgError::{RangesAreOutOfBounds, WrongLowerBound, WrongUpperBound};
use crate::utils::{contains, OxAgError, MAP_RANGE};

#[derive(Debug, Clone)]
pub struct OxAgWorldGenerationOptions {
    pub deep_water_level: RangeInclusive<f64>,
    pub shallow_water_level: RangeInclusive<f64>,
    pub sand_level: RangeInclusive<f64>,
    pub grass_level: RangeInclusive<f64>,
    pub hill_level: RangeInclusive<f64>,
    pub mountain_level: RangeInclusive<f64>,
    pub snow_level: RangeInclusive<f64>,
    // other implementation for street and lava tiles
}

impl OxAgWorldGenerationOptions {
    pub fn new(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);

        let dw_end = rng.gen_range(MAP_RANGE);
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
            Err(WrongLowerBound)?
        }
        // at least one with end = 1.0
        if !levels.iter().any(|l| *l.end() == 1.0) {
            Err(WrongUpperBound)?
        }
        // all must be between -1.0 and 1.0        }
        if levels.iter().any(|l| !contains(&MAP_RANGE, l)) {
            Err(RangesAreOutOfBounds)?
        }
        Ok(())
    }
    pub fn from_preset(preset: OxAgWorldGenerationPresets) -> Self {
        match preset {
            OxAgWorldGenerationPresets::DEFAULT => presets::DEFAULT,
            OxAgWorldGenerationPresets::WATERWORLD => presets::WATER_WORLD,
            OxAgWorldGenerationPresets::LOWWATERWORLD => presets::LOW_WATER_WORLD,
        }
    }
}

/// # Presets
/// List of available presets
///
/// <pre style="color: orange;">
/// ┌──────────────────────┬───────────────────────┐
/// │     Parameter        │     Description       │
/// ├──────────────────────┼───────────────────────┤
/// │ deep_water_level     │ Deep water            │
/// │ shallow_water_level  │ Shallow water         │
/// │ sand_level           │ Sandy areas           │
/// │ grass_level          │ Grassy terrain        │
/// │ hill_level           │ Hilly landscapes      │
/// │ mountain_level       │ Mountainous regions   │
/// │ snow_level           │ Snowy landscapes      │
/// └──────────────────────┴───────────────────────┘
/// </pre>
///
/// # Entries
/// - [DEFAULT](enum.OxAgWorldGenerationPresets.html#variant.DEFAULT)
/// - [WATERWORLD](enum.OxAgWorldGenerationPresets.html#variant.WATERWORLD)
/// - [LOWWATERWORLD](enum.OxAgWorldGenerationPresets.html#variant.LOWWATERWORLD)
///
#[derive(Copy, Clone, Debug)]
pub enum OxAgWorldGenerationPresets {
    /// # Default world generation option
    /// <pre style="color: orange;">
    /// ┌──────────────────────┬─────────────────┐
    /// │     Parameter        │   Value Range   │
    /// ├──────────────────────┼─────────────────┤
    /// │ deep_water_level     │ -1.0  ..= -0.75 │
    /// │ shallow_water_level  │ -0.75 ..= -0.5  │
    /// │ sand_level           │ -0.5  ..= -0.25 │
    /// │ grass_level          │ -0.25 ..=  0.25 │
    /// │ hill_level           │  0.25 ..=  0.5  │
    /// │ mountain_level       │  0.5  ..=  0.75 │
    /// │ snow_level           │  0.75 ..=  1.0  │
    /// └──────────────────────┴─────────────────┘
    /// </pre>
    /// [`PRESETS`](OxAgWorldGenerationPresets)
    DEFAULT,
    ///
    /// # Water world generation option
    /// <pre style="color: orange;">
    /// ┌──────────────────────┬─────────────────┐
    /// │     Parameter        │   Value Range   │
    /// ├──────────────────────┼─────────────────┤
    /// │ deep_water_level     │ -1.0  ..= -0.5  │
    /// │ shallow_water_level  │ -0.5  ..=  0.0  │
    /// │ sand_level           │  0.0  ..=  0.2  │
    /// │ grass_level          │  0.2  ..=  0.4  │
    /// │ hill_level           │  0.4  ..=  0.6  │
    /// │ mountain_level       │  0.6  ..=  0.8  │
    /// │ snow_level           │  0.8  ..=  1.0  │
    /// └──────────────────────┴─────────────────┘
    /// </pre>
    /// [`PRESETS`](OxAgWorldGenerationPresets)
    WATERWORLD,
    ///
    /// # Low water world generation option
    /// <pre style="color: orange;">
    /// ┌──────────────────────┬─────────────────┐
    /// │     Parameter        │   Value Range   │
    /// ├──────────────────────┼─────────────────┤
    /// │ deep_water_level     │ -1.0  ..= -0.8  │
    /// │ shallow_water_level  │ -0.8  ..= -0.6  │
    /// │ sand_level           │ -0.6  ..= -0.3  │
    /// │ grass_level          │ -0.3  ..=  0.1  │
    /// │ hill_level           │  0.1  ..=  0.4  │
    /// │ mountain_level       │  0.4  ..=  0.7  │
    /// │ snow_level           │  0.7  ..=  1.0  │
    /// └──────────────────────┴─────────────────┘
    /// </pre>
    /// [`PRESETS`](OxAgWorldGenerationPresets)
    LOWWATERWORLD,
}

pub(crate) mod presets {
    use crate::worldgenerator::world_options::OxAgWorldGenerationOptions;

    pub const DEFAULT: OxAgWorldGenerationOptions = OxAgWorldGenerationOptions {
        deep_water_level: -1.0..=-0.75,
        shallow_water_level: -0.75..=-0.5,
        sand_level: -0.5..=-0.25,
        grass_level: -0.25..=0.25,
        hill_level: 0.25..=0.5,
        mountain_level: 0.5..=0.75,
        snow_level: 0.75..=1.0,
    };
    pub const WATER_WORLD: OxAgWorldGenerationOptions = OxAgWorldGenerationOptions {
        deep_water_level: -1.0..=-0.5,
        shallow_water_level: -0.5..=0.0,
        sand_level: 0.0..=0.2,
        grass_level: 0.2..=0.4,
        hill_level: 0.4..=0.6,
        mountain_level: 0.6..=0.8,
        snow_level: 0.8..=1.0,
    };
    pub const LOW_WATER_WORLD: OxAgWorldGenerationOptions = OxAgWorldGenerationOptions {
        deep_water_level: -1.0..=-0.8,
        shallow_water_level: -0.8..=-0.6,
        sand_level: -0.6..=-0.3,
        grass_level: -0.3..=0.1,
        hill_level: 0.1..=0.4,
        mountain_level: 0.4..=0.7,
        snow_level: 0.7..=1.0,
    };
}