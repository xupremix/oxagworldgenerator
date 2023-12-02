use std::ops::RangeInclusive;

use rand::prelude::StdRng;
use rand::Rng;
use rand::SeedableRng;

use crate::utils::constants::*;
use crate::utils::errors::OxAgError;
use crate::utils::Container;

/// Levels that will determine the spawn of the different tile types.
/// TODO: Examples
#[derive(Debug, Clone)]
pub struct OxAgTileTypeSpawnLevels {
    pub deep_water_level: RangeInclusive<f64>,
    pub shallow_water_level: RangeInclusive<f64>,
    pub sand_level: RangeInclusive<f64>,
    pub grass_level: RangeInclusive<f64>,
    pub hill_level: RangeInclusive<f64>,
    pub mountain_level: RangeInclusive<f64>,
    pub snow_level: RangeInclusive<f64>,
    // TODO: lava & street tiles?
}

impl OxAgTileTypeSpawnLevels {
    /// Calculates the spawn levels via a given `seed`.
    pub fn new_from_seed(seed: u64) -> Self {
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

    /// Returns a [OxAgTileTypeSpawnLevels] from a given `preset`
    pub fn from_preset(preset: OxAgTileTypeSpawnLevelPresets) -> Self {
        match preset {
            OxAgTileTypeSpawnLevelPresets::DEFAULT => presets::DEFAULT,
            OxAgTileTypeSpawnLevelPresets::WATERWORLD => presets::WATER_WORLD,
            OxAgTileTypeSpawnLevelPresets::LOWWATERWORLD => presets::LOW_WATER_WORLD,
            OxAgTileTypeSpawnLevelPresets::HILL => presets::HILL,
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
/// - [DEFAULT](enum.OxAgTileTypeSpawnLevelPresets.html#variant.DEFAULT)
/// - [WATERWORLD](enum.OxAgTileTypeSpawnLevelPresets.html#variant.WATERWORLD)
/// - [LOWWATERWORLD](enum.OxAgTileTypeSpawnLevelPresets.html#variant.LOWWATERWORLD)
///
#[derive(Copy, Clone, Debug)]
pub enum OxAgTileTypeSpawnLevelPresets {
    /// # Default tile type spawn levels
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
    /// [`PRESETS`](OxAgTileTypeSpawnLevelPresets)
    DEFAULT,
    ///
    /// # Water tile type spawn levels
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
    /// [`PRESETS`](OxAgTileTypeSpawnLevelPresets)
    WATERWORLD,
    ///
    /// # Low water tile type spawn levels
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
    /// [`PRESETS`](OxAgTileTypeSpawnLevelPresets)
    LOWWATERWORLD,
    ///
    /// # Low water tile type spawn levels
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
    /// [`PRESETS`](OxAgTileTypeSpawnLevelPresets)
    HILL,
}

pub(crate) mod presets {
    use crate::world_generator::tile_type_spawn_levels::OxAgTileTypeSpawnLevels;

    pub const DEFAULT: OxAgTileTypeSpawnLevels = OxAgTileTypeSpawnLevels {
        deep_water_level: -1.0..=-0.75,
        shallow_water_level: -0.75..=-0.5,
        sand_level: -0.5..=-0.25,
        grass_level: -0.25..=0.25,
        hill_level: 0.25..=0.5,
        mountain_level: 0.5..=0.75,
        snow_level: 0.75..=1.0,
    };

    pub const WATER_WORLD: OxAgTileTypeSpawnLevels = OxAgTileTypeSpawnLevels {
        deep_water_level: -1.0..=-0.5,
        shallow_water_level: -0.5..=0.0,
        sand_level: 0.0..=0.2,
        grass_level: 0.2..=0.4,
        hill_level: 0.4..=0.6,
        mountain_level: 0.6..=0.8,
        snow_level: 0.8..=1.0,
    };

    pub const LOW_WATER_WORLD: OxAgTileTypeSpawnLevels = OxAgTileTypeSpawnLevels {
        deep_water_level: -1.0..=-0.8,
        shallow_water_level: -0.8..=-0.6,
        sand_level: -0.6..=-0.3,
        grass_level: -0.3..=0.1,
        hill_level: 0.1..=0.4,
        mountain_level: 0.4..=0.7,
        snow_level: 0.7..=1.0,
    };

    pub const HILL: OxAgTileTypeSpawnLevels = OxAgTileTypeSpawnLevels {
        deep_water_level: -1.0..=-1.0,
        shallow_water_level: -1.0..=-1.0,
        sand_level: -1.0..=-1.0,
        grass_level: -1.0..=-1.0,
        hill_level: -1.0..=1.0,
        mountain_level: 1.0..=1.0,
        snow_level: 1.0..=1.0,
    };
}
