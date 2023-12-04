use crate::utils::traits::Loadable;
use crate::world_generator::tile_type_options::OxAgTileTypeOptions;

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
/// - [Default](enum.OxAgTileTypeSpawnPresets.html#variant.Default)
/// - [WaterWorld](enum.OxAgTileTypeSpawnPresets.html#variant.WaterWorld)
/// - [LowWaterWorld](enum.OxAgTileTypeSpawnPresets.html#variant.LowWaterWorld)
#[derive(Copy, Clone, Debug)]
pub enum OxAgTileTypePresets {
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
    /// [`PRESETS`](OxAgTileTypePresets)
    Default,
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
    /// [`PRESETS`](OxAgTileTypePresets)
    WaterWorld,
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
    /// [`PRESETS`](OxAgTileTypePresets)
    LowWaterWorld,
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
    /// [`PRESETS`](OxAgTileTypePresets)
    Hill,
}

impl Loadable<OxAgTileTypeOptions> for OxAgTileTypePresets {
    fn load(&self) -> OxAgTileTypeOptions {
        match self {
            OxAgTileTypePresets::Default => DEFAULT,
            OxAgTileTypePresets::WaterWorld => WATER_WORLD,
            OxAgTileTypePresets::LowWaterWorld => LOW_WATER_WORLD,
            OxAgTileTypePresets::Hill => HILL,
        }
    }
}

pub(crate) const DEFAULT: OxAgTileTypeOptions = OxAgTileTypeOptions {
    deep_water_level: -1.0..=-0.75,
    shallow_water_level: -0.75..=-0.5,
    sand_level: -0.5..=-0.25,
    grass_level: -0.25..=0.25,
    hill_level: 0.25..=0.5,
    mountain_level: 0.5..=0.75,
    snow_level: 0.75..=1.0,
};

pub const WATER_WORLD: OxAgTileTypeOptions = OxAgTileTypeOptions {
    deep_water_level: -1.0..=-0.5,
    shallow_water_level: -0.5..=0.0,
    sand_level: 0.0..=0.2,
    grass_level: 0.2..=0.4,
    hill_level: 0.4..=0.6,
    mountain_level: 0.6..=0.8,
    snow_level: 0.8..=1.0,
};

pub const LOW_WATER_WORLD: OxAgTileTypeOptions = OxAgTileTypeOptions {
    deep_water_level: -1.0..=-0.8,
    shallow_water_level: -0.8..=-0.6,
    sand_level: -0.6..=-0.3,
    grass_level: -0.3..=0.1,
    hill_level: 0.1..=0.4,
    mountain_level: 0.4..=0.7,
    snow_level: 0.7..=1.0,
};

pub const HILL: OxAgTileTypeOptions = OxAgTileTypeOptions {
    deep_water_level: -1.0..=-1.0,
    shallow_water_level: -1.0..=-1.0,
    sand_level: -1.0..=-1.0,
    grass_level: -1.0..=-1.0,
    hill_level: -1.0..=1.0,
    mountain_level: 1.0..=1.0,
    snow_level: 1.0..=1.0,
};
